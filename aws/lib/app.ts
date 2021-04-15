import * as cdk from '@aws-cdk/core';
import { DatabaseCluster, ClusterParameterGroup } from "@aws-cdk/aws-docdb";
import { InstanceType, InstanceClass, InstanceSize, SubnetType, Port } from "@aws-cdk/aws-ec2";
import { Role } from "@aws-cdk/aws-iam";
import { Vpc } from "@aws-cdk/aws-ec2";
import { ContainerImage, Cluster, FargateService, FargateTaskDefinition, Protocol } from "@aws-cdk/aws-ecs";
import { Repository } from "@aws-cdk/aws-ecr";
import { Secret, ISecret } from "@aws-cdk/aws-secretsmanager";
import { Certificate } from "@aws-cdk/aws-certificatemanager";
import { ApplicationProtocol, ApplicationLoadBalancer, ApplicationTargetGroup } from "@aws-cdk/aws-elasticloadbalancingv2";


export interface SharedResources extends cdk.StackProps {
  deploymentRole: Role,
  vpc: Vpc,
  repository: Repository,
}


export class AppStack extends cdk.Stack {
  databaseCluster: DatabaseCluster;
  ecsCluster: Cluster;
  databaseSecret: ISecret;
  taskDefinition: FargateTaskDefinition;
  service: FargateService;
  targetGroup: ApplicationTargetGroup;

  constructor(scope: cdk.Construct, id: string, props: SharedResources) {
    super(scope, id, props);

    // Database Password
    // TODO: Waiting on https://github.com/aws/aws-cdk/issues/14110
    this.databaseSecret = Secret.fromSecretNameV2(this, 'database-credentials-secret', 'prod/todos/DatabaseCredentials');

    // Database Param Group
    const paramGroup = new ClusterParameterGroup(this, 'todos-database-param-group', {
      family: "docdb4.0",
      parameters: {
        "audit_logs": "disabled",
        "change_stream_log_retention_duration": "10800",
        "profiler": "disabled",
        "profiler_sampling_rate": "1.0",
        "profiler_threshold_ms": "100",
        "tls": "disabled", // Do not add this in Production...
        "ttl_monitor": "enabled"
      }
    });

    // Create the Database
    this.databaseCluster = new DatabaseCluster(this, 'todos-database', {
      masterUser: {
        username: this.databaseSecret.secretValueFromJson("username").toString(), // NOTE: 'admin' is reserved by DocumentDB
        password: this.databaseSecret.secretValueFromJson("password")
      },
      engineVersion: "4.0.0",
      instanceType: InstanceType.of(InstanceClass.T3, InstanceSize.MEDIUM),
      vpcSubnets: {
        subnetType: SubnetType.PUBLIC,
      },
      vpc: props.vpc,
      dbClusterName: "todos-database-cluster",
      removalPolicy: cdk.RemovalPolicy.DESTROY, // Do not add this in Production...
      instances: 1,
      parameterGroup: paramGroup
    });
    // Do not add this in Production...
    this.databaseCluster.connections.allowDefaultPortFromAnyIpv4();

    // Create the ECS Cluster
    this.ecsCluster = new Cluster(this, "todos-cluster", {
      clusterName: "todos-cluster",
      vpc: props.vpc,
    });

    // TLS Cert
    const cert = Certificate.fromCertificateArn(this, 'load-balancer-cert', `arn:aws:acm:${this.region}:${this.account}:certificate/97b02803-9d97-4b30-b18a-65f1eb8c1cc0`);

    // Load Balancer
    const loadBalancer = new ApplicationLoadBalancer(this, 'load-balancer', {
      vpc: props.vpc,
      internetFacing: true,
      deletionProtection: false
    });

    const httpListener = loadBalancer.addListener('http-listener', {
      protocol: ApplicationProtocol.HTTP,
      port: 80
    });

    // TODO: Remove Deprecation
    httpListener.addRedirectResponse('http-redirect', {
      statusCode: 'HTTP_301',
      protocol: ApplicationProtocol.HTTPS,
      port: '443',
    });

    const httpsListener = loadBalancer.addListener('https-listener', {
      protocol: ApplicationProtocol.HTTPS,
      port: 443,
      certificates: [cert]
    });

    // Task Definition
    this.taskDefinition = new FargateTaskDefinition(this, 'todos-task-definition', {
      cpu: 512,
      memoryLimitMiB: 2048
    });
    
    const connectionString = `mongodb://${this.databaseSecret.secretValueFromJson("username")}:${this.databaseSecret.secretValueFromJson("password")}@${this.databaseCluster.clusterEndpoint.hostname}:27017/?ssl=false&replicaSet=rs0&readPreference=secondaryPreferred&retryWrites=false`;
    this.taskDefinition.addContainer("task-definition-container", {
      image: ContainerImage.fromEcrRepository(props.repository),
      environment: {
        "AUTHORITY": "https://dev-cy9xf-r5.us.auth0.com/",
        "DATABASE_CONNECTION_STRING": connectionString
      },
      portMappings: [{
        containerPort: 8080,
        hostPort: 8080,
        protocol: Protocol.TCP
      }]
    });

    this.service = new FargateService(this, 'fargate-service', {
      taskDefinition: this.taskDefinition,
      assignPublicIp: true,
      cluster: this.ecsCluster,
      desiredCount: 1
    });

    this.targetGroup = new ApplicationTargetGroup(this, "application-target-group", {
      port: 8080,
      protocol: ApplicationProtocol.HTTP,
      vpc: props.vpc,
      targets: [this.service]
    });

    httpsListener.addTargetGroups('https-target-groups', {
      targetGroups: [this.targetGroup]
    });
  }
}