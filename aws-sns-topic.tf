resource "aws_sns_topic" "rdf_load" {
  provider = aws.ekg_api
  name     = local.sns_topic

  # Make sure that SNS can log to CloudWatch for each Lambda invocation
  lambda_failure_feedback_role_arn = aws_iam_role.sns_feedback_role.arn
  lambda_success_feedback_role_arn = aws_iam_role.sns_feedback_role.arn

  lambda_success_feedback_sample_rate = 100

  policy = data.aws_iam_policy_document.s3_to_sns.json

  tags = local.default_tags

  depends_on = [
    aws_lambda_function.invoke
  ]
}

# Subscribe the lambda function to the SNS topic it handles
resource "aws_sns_topic_subscription" "rdf_load" {
  topic_arn = aws_sns_topic.rdf_load.arn
  protocol  = "lambda"
  endpoint  = aws_lambda_function.invoke.arn
}
