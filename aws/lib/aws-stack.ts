import * as cdk from '@aws-cdk/core';
import { Repository } from "@aws-cdk/aws-ecr";
import { User, Role, Policy, PolicyStatement, Effect } from "@aws-cdk/aws-iam";
import { Vpc } from "@aws-cdk/aws-ec2";

export class AwsStack extends cdk.Stack {
  repository: Repository;
  githubDeploymentRole: Role;
  vpc: Vpc;

  constructor(scope: cdk.Construct, id: string, props: cdk.StackProps) {
    super(scope, id, props);

    // Container Registry
    this.repository = new Repository(this, "todos-repository", {
      removalPolicy: cdk.RemovalPolicy.DESTROY
    });

    // GitHub Deployment User
    const githubDeploymentUser = new User(this, "github-deployment-user");

    // GitHub Deployment Role
    this.githubDeploymentRole = new Role(this, "github-deployment-role", {
      assumedBy: githubDeploymentUser
    });

    this.githubDeploymentRole.assumeRolePolicy?.addStatements(new PolicyStatement({
      principals: [ githubDeploymentUser ],
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
          resources: [ this.githubDeploymentRole.roleArn ],
          effect: Effect.ALLOW
        })
      ]
    }));

    // Grant permission to Push/Pull images from GitHub
    this.repository.grantPullPush(this.githubDeploymentRole);

    // Deployment VPC
    this.vpc = new Vpc(this, "todos-vpc", {
      cidr: "10.0.0.0/16"
    });

    cdk.Tags.of(this.vpc).add("Name", "todos-vpc");
  }
}
