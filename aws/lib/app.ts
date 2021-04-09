import * as cdk from '@aws-cdk/core';
import { DatabaseCluster } from "@aws-cdk/aws-docdb";
import { InstanceType, InstanceClass, InstanceSize, SubnetType } from "@aws-cdk/aws-ec2";
import { Role } from "@aws-cdk/aws-iam";
import { Vpc } from "@aws-cdk/aws-ec2";


export interface SharedResources extends cdk.StackProps {
  deploymentRole: Role,
  vpc: Vpc,
}


export class AppStack extends cdk.Stack {

  constructor(scope: cdk.Construct, id: string, props: SharedResources) {
    super(scope, id, props);

    // Create the Database
    const cluster = new DatabaseCluster(this, 'todos-database', {
        masterUser: {
            username: 'myuser' // NOTE: 'admin' is reserved by DocumentDB
        },
        instanceType: InstanceType.of(InstanceClass.M3, InstanceSize.MEDIUM),
        vpcSubnets: {
            subnetType: SubnetType.PRIVATE,
        },
        vpc: props.vpc
    });
  }
}