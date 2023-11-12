# AWS Lambda Functions

## Overview

This directory contains all the various AWS Lambda functions that are used for
the Digital Twin architecture. Each function is contained in its own directory.

The name of the lambda function is equal to its directory name, for
example, the `rdf_load` function is in the directory `lambda/rdf_load`.

## Deployment

Each lambda function is deployed using Terraform in the [`infra` directory](../infra/README.md).

## Lambda Functions

- [`rdf_load`](rdf_load/README.md)
