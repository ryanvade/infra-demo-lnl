#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from '@aws-cdk/core';
import { AwsStack } from '../lib/aws-stack';
import { AppStack } from '../lib/app';

const app = new cdk.App();
const sharedStack = new AwsStack(app, 'AwsStack', {
  /* For more information, see https://docs.aws.amazon.com/cdk/latest/guide/environments.html */
});

const appStack = new AppStack(app, "AppStack", {
  deploymentRole: sharedStack.githubDeploymentRole,
  vpc: sharedStack.vpc
})
