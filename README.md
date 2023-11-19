# Terraform AWS Neptune RDF Load

Load RDF files (.nt or .ttl files only for now) from a given S3 bucket into AWS Neptune.

The [`rdf_load` lambda function](./lambda/rdf-load/README.md) in this repository picks up the S3 ObjectCreated
event and passes the S3 file to the Neptune loader service.

## Other documentation

- [Other EKGF Terraform modules](https://registry.terraform.io/namespaces/EKGF)
- [Generated documentation for this module](https://registry.terraform.io/modules/EKGF/rdf-load/aws/latest)

## Things to improve

- The Python code in the lambda function is not very robust. It should be improved to handle errors better.
- The lambda function should be able to handle multiple files per incoming SNS event.
- The lambda function should be able to handle multiple S3 buckets.
    - This is already possible but since we probably end up with different access policies for these input buckets
      anyway, each bucket should have its own lambda function with just access to that single bucket.
- The code for the lambda function should only be built once.
    - Currently, the code is built for each deployment of the rdf-load module.
    - This is not a big problem since the code is small and the build is fast, but it should be improved.
        - The code is built using the `build.sh` script in the `lambda/rdf-load` directory and usually takes <3s
    - The code should be built once and then uploaded to S3 and referenced from there by the lambda function.
    - One option is to publish the python code as a package to Pypi.org and then reference it from there.
        - This would still require a small wrapper python script to be built as "artifact.zip"
        - Ideally, the whole artifact.zip file itself would be published to Pypi.org or some other public accessible
          place and referenced from there.