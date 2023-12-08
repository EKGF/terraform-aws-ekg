# Lambda function `invoke`

This lambda function is triggered via SNS topic `rdf_load` by a
given S3 bucket.
The trigger happens whenever a new RDF file is created, updated, or
has its tags updated, in the given bucket.

This lambda function invokes the RDF Load Step Function which will
then load the RDF file into the given Neptune cluster.

## Forced reloads

One "trick" to force a reload of a given RDF file in a bucket is
to manually increase its `reloaded` tag using the AWS Console or
the AWS CLI.

