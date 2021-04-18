import * as cdk from '@aws-cdk/core';
import { Repository } from "@aws-cdk/aws-ecr";
import { User, Role, Policy, PolicyStatement, Effect } from "@aws-cdk/aws-iam";
import { Vpc } from "@aws-cdk/aws-ec2";

export class AwsStack extends cdk.Stack {
  repository: Repository;
  githubDeploymentRole: Role;
  githubReadOnlyrole: Role;
  vpc: Vpc;

  constructor(scope: cdk.Construct, id: string, props: cdk.StackProps) {
    super(scope, id, props);

    // Container Registry
    this.repository = new Repository(this, "todos-repository", {
      removalPolicy: cdk.RemovalPolicy.DESTROY,
      repositoryName: "todos-repository"
    });

    // GitHub Deployment User
    const githubDeploymentUser = new User(this, "github-deployment-user");

    // GitHub Deployment Role
    this.githubDeploymentRole = new Role(this, "github-deployment-role", {
      assumedBy: githubDeploymentUser
    });

    this.githubDeploymentRole.assumeRolePolicy?.addStatements(new PolicyStatement({
      principals: [githubDeploymentUser],
      actions: ["sts:TagSession"],
      effect: Effect.ALLOW
    }));

    githubDeploymentUser.attachInlinePolicy(new Policy(this, 'github-deployment-user-assume-role-policy', {
      statements: [
        new PolicyStatement({
          actions: [
            "sts:AssumeRole",
            "sts:TagSession"
          ],
          resources: [this.githubDeploymentRole.roleArn],
          effect: Effect.ALLOW
        })
      ]
    }));

    // Grant permission to Push/Pull images from GitHub
    this.repository.grantPullPush(this.githubDeploymentRole);

    // Read Only Role (Synth + Diff)
    this.githubReadOnlyrole = new Role(this, "github-read-only-role", {
      assumedBy: githubDeploymentUser
    });

    this.githubReadOnlyrole.assumeRolePolicy?.addStatements(new PolicyStatement({
      principals: [githubDeploymentUser],
      actions: ["sts:TagSession"],
      effect: Effect.ALLOW
    }));

    // ECR Read Only
    this.githubReadOnlyrole.addToPolicy(new PolicyStatement({
      actions: [
        "ecr:DescribeImages",
        "ecr:DescribeRepositories",
        "ecr:ListTagsForResource",
        "ecr:ListImages",
        "ecr:GetRepositoryPolicy",
        "ecr:GetLifecyclePolicy"
      ],
      resources: [
        `arn:aws:ecr:${this.region}:${this.account}:repository/todos*`
      ]
    }));

    this.githubReadOnlyrole.addToPolicy(new PolicyStatement({
      actions: [
        "ecs:DescribeCapacityProviders",
        "ecs:ListTagsForResource",
        "ecs:ListAttributes",
        "ecs:ListTasks",
        "ecs:DescribeServices",
        "ecs:DescribeTaskSets",
        "ecs:ListContainerInstances",
        "ecs:DescribeContainerInstances",
        "ecs:DescribeTasks",
        "ecs:DescribeClusters"
      ],
      resources: [
        `arn:aws:ecs:${this.region}:${this.account}:task/*`,
        `arn:aws:ecs:${this.region}:${this.account}:container-instance/*`,
        `arn:aws:ecs:${this.region}:${this.account}:service/todos*`,
        `arn:aws:ecs:${this.region}:${this.account}:task-definition/*:*`,
        `arn:aws:ecs:${this.region}:${this.account}:cluster/todos*`,
        `arn:aws:ecs:${this.region}:${this.account}:task-set/*/*/*`,
        `arn:aws:ecs:${this.region}:${this.account}:capacity-provider/*`
      ]
    }));

    this.githubReadOnlyrole.addToPolicy(new PolicyStatement({
      actions: [
        "ecs:ListServices",
        "ecs:ListAccountSettings",
        "ecs:ListTaskDefinitionFamilies",
        "ecs:ListTaskDefinitions",
        "ecs:DescribeTaskDefinition",
        "ecs:ListClusters"
      ],
      resources: [
        "*"
      ]
    }));

    this.githubReadOnlyrole.addToPolicy(new PolicyStatement({
      actions: [
        "cloudformation:DescribeStackEvents",
        "cloudformation:DescribeStackSet",
        "cloudformation:ListStackSets",
        "cloudformation:DescribeStackInstance",
        "cloudformation:DescribeStackResources",
        "cloudformation:DescribeStackResource",
        "cloudformation:DescribeChangeSet",
        "cloudformation:DescribeStacks",
        "cloudformation:GetTemplate"
      ],
      resources: [
        `arn:aws:cloudformation:${this.region}:${this.account}:stackset/*/*`,
        `arn:aws:cloudformation:${this.region}:${this.account}:stack/*/*`
      ]
    }));

    this.githubReadOnlyrole.addToPolicy(new PolicyStatement({
      actions: [
        "cloudformation:ListStacks",
        "cloudformation:DescribeType",
        "cloudformation:ValidateTemplate"
      ],
      resources: [
        "*"
      ]
    }));

    this.githubReadOnlyrole.addToPolicy(new PolicyStatement({
      actions: [
        "ec2:DescribeAvailabilityZones",
        "ec2:DescribeSecurityGroupReferences",
        "ec2:DescribeTags",
        "ec2:DescribeVpcs",
        "ec2:DescribeNatGateways",
        "ec2:DescribeSubnets",
        "ec2:DescribeNetworkAcls",
        "ec2:DescribeVpcAttribute",
        "ec2:DescribeRouteTables",
        "ec2:DescribeVpnGateways",
        "ec2:DescribeSecurityGroups"
      ],
      resources: [
        "*"
      ]
    }));

    this.githubReadOnlyrole.addToPolicy(new PolicyStatement({
      actions: [
        "secretsmanager:DescribeSecret"
      ],
      resources: [
        `arn:aws:secretsmanager:${this.region}:${this.account}:secret:*`
      ]
    }));

    this.githubReadOnlyrole.addToPolicy(new PolicyStatement({
      actions: [
        "secretsmanager:ListSecrets"
      ],
      resources: [
        "*"
      ]
    }));

    this.githubReadOnlyrole.addToPolicy(new PolicyStatement({
      actions: [
        "acm:DescribeCertificate",
        "acm:GetCertificate"
      ],
      resources: [
        `arn:aws:acm:${this.region}:${this.account}:certificate/*`
      ]
    }));

    this.githubReadOnlyrole.addToPolicy(new PolicyStatement({
      actions: [
        "acm:ListCertificates"
      ],
      resources: [
        "*"
      ]
    }));

    // Deployment VPC
    this.vpc = new Vpc(this, "todos-vpc", {
      cidr: "10.0.0.0/16",
      natGateways: 0,
    });

    cdk.Tags.of(this.vpc).add("Name", "todos-vpc");
  }
}
