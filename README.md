# Terraform module AWS EKG

This Terraform module is a core part of running an Enterprise Knowledge Graph (EKG) architecture on AWS.

## RDF Load

Loads RDF files (.nt or .ttl files only for now) from a given S3 bucket into AWS Neptune.

This Terraform module uses an AWS Step Function to orchestrate the loading of a given RDF file into Neptune.
Any RDF file (.nt or .ttl) that is uploaded to the given S3 bucket will be trigger an SNS event picked up
by the [invoke](./crate/ekg-lfn-invoke/README.md) lambda function.
This lambda function will then start the Step Function that first instructs the Neptune bulk loader to load the file
(using the [load](./crate/ekg-lfn-load/README.md)) and then polls Neptune to check if the loading is done using
the [check](./crate/ekg-lfn-check/README.md) lambda function.

## Other documentation

- [Other EKGF Terraform modules](https://registry.terraform.io/namespaces/EKGF)
- [Generated documentation for this module](https://registry.terraform.io/modules/EKGF/rdf-load/aws/latest)

## Things to improve

- [ ] Create a mockup server that mimics the Neptune loader service so that we can run the test
- [ ] Reduce the amount of logging down to the essentials
- [ ] Support Excel files, run them through a lambda function that converts them to "Raw RDF" files
- [ ] Support CSV files, run them through a lambda function that converts them to "Raw RDF" files
- [ ] Support "the Story Service", executing stories as defined per Use Case