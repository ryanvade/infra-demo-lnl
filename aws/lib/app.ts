import * as cdk from '@aws-cdk/core';
import { DatabaseCluster, DatabaseSecret } from "@aws-cdk/aws-docdb";
import { InstanceType, InstanceClass, InstanceSize, SubnetType, Port } from "@aws-cdk/aws-ec2";
import { Role } from "@aws-cdk/aws-iam";
import { Vpc } from "@aws-cdk/aws-ec2";
import { Cluster } from "@aws-cdk/aws-ecs";
import { ApplicationLoadBalancedFargateService } from "@aws-cdk/aws-ecs-patterns";
import { ContainerImage } from "@aws-cdk/aws-ecs";
import { Repository } from "@aws-cdk/aws-ecr";


export interface SharedResources extends cdk.StackProps {
  deploymentRole: Role,
  vpc: Vpc,
  repository: Repository,
}


export class AppStack extends cdk.Stack {
  databaseCluster: DatabaseCluster;
  ecsCluster: Cluster;
  fargateService: ApplicationLoadBalancedFargateService;
  clusterUsername: "todos-user";
  databaseSecret: DatabaseSecret;

  constructor(scope: cdk.Construct, id: string, props: SharedResources) {
    super(scope, id, props);

    // Create the Database
    this.databaseCluster = new DatabaseCluster(this, 'todos-database', {
        masterUser: {
            username: this.clusterUsername, // NOTE: 'admin' is reserved by DocumentDB
        },
        instanceType: InstanceType.of(InstanceClass.M3, InstanceSize.MEDIUM),
        vpcSubnets: {
            subnetType: SubnetType.PRIVATE,
        },
        vpc: props.vpc
    });
    // Setup Secret
    this.databaseSecret =  new DatabaseSecret(this, 'todos-database-secret', {
      username: this.clusterUsername,
      masterSecret: this.databaseCluster.secret
    });
    // Never do this
    this.databaseCluster.connections.allowDefaultPortFromAnyIpv4();

    // Create the ECS Cluster
    this.ecsCluster = new Cluster(this, "todos-cluster", {
      clusterName: "todos-cluster",
      vpc: props.vpc,
    });

    const connectionString = `mongodb://${this.clusterUsername}:${this.databaseSecret.secretValue}@${this.databaseCluster.clusterEndpoint.socketAddress}?replicaSet=rs0&ssl_ca_certs=rds-combined-ca-bundle.pem`;
    // Create the Fargate Service
    this.fargateService = new ApplicationLoadBalancedFargateService(this, "todos-fargate-service", {
      cluster: this.ecsCluster, // Required
      cpu: 512, // Default is 256
      desiredCount: 2, // Default is 1
      taskImageOptions: { 
        image: ContainerImage.fromEcrRepository(props.repository),
        environment: {
          "AUTHORITY": "https://dev-cy9xf-r5.us.auth0.com/",
          "DATABASE_CONNECTION_STRING": connectionString
        }
      },
      memoryLimitMiB: 2048, // Default is 512
      publicLoadBalancer: true, // Default is false
    });
  }
}