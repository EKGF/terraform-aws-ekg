# Terraform AWS Neptune RDF Load

Load RDF files (.nt or .ttl files only for now) from a given S3 bucket into AWS Neptune.

The [`rdf_load` lambda function](./lambda/rdf-load/README.md) in this repository picks up the S3 ObjectCreated
event and passes the S3 file to the Neptune loader service.

## Other documentation

- [Other EKGF Terraform modules](https://registry.terraform.io/namespaces/EKGF)
- [Generated documentation for this module](https://registry.terraform.io/modules/EKGF/rdf-load/aws/latest)
