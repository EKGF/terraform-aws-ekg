# Terraform AWS Neptune RDF Load

Load RDF files (.nt or .ttl files only for now) from a given S3 bucket into AWS Neptune.

The [`invoke` lambda function](./lambda/invoke/README.md) in this repository picks
up the S3 ObjectCreated event and passes it to an AWS Step Function that orchestrates
the loading of the given RDF file into Neptune.

The [`load` lambda function](./lambda/load/README.md) in this repository gets called
by the Step Function and instructs the Neptune bulk loader to load the given S3 file.

## Other documentation

- [Other EKGF Terraform modules](https://registry.terraform.io/namespaces/EKGF)
- [Generated documentation for this module](https://registry.terraform.io/modules/EKGF/rdf-load/aws/latest)

## Things to improve

- [ ] Move the code for the lambda's to the
  [ekglib](https://github.com/EKGF/ekglib) library.
  The code for the lambda functions should only be built once.
    - Currently, the code is built for each deployment of the rdf-load module.
        - This is not a big problem since the code is small and the build is
          fast, but it should be improved. It also requires Python and Poetry etc
          to be installed on the machine that runs the terraform code.
        - The code is built using the `build.sh` script in each directory
          under `lambda/` and usually takes <3s
- [ ] Create a mockup server that mimics the Neptune loader service
  so that we can run the test
- [ ] The Python code in the lambda functions is currently not very robust.
  It should be improved to handle errors better.
- [ ] Reduce the amount of logging down to the essentials
- [ ] Support Excel files, run them through a lambda function that
  converts them to "Raw RDF" files using
  [the XlsxParser in the ekglib library](https://github.com/EKGF/ekglib/tree/main/ekglib/xlsx_parser)
