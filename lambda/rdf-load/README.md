# Lambda function `rdf_load`

This lambda function is triggered via SNS topic `rdf_load` by a given S3 bucket.
The trigger happens whenever a new RDF file is created (or its tags are changed) in the
given bucket.

## Force reloads

One "trick" to force a reload of a given RDF file in a bucket is to manually increase
its `reloaded` tag in the AWS Console.

## TODOs

- [ ] Create a mockup server that mimicks the Neptune loader service so that we can run the test
- [ ] Reduce the amount of logging down to the essentials
- [ ] Trigger another SNS topic for an AWS Step Function to then monitor the loader status