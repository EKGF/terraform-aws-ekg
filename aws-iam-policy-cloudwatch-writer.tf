data "aws_iam_policy_document" "cloudwatch_writer" {
  statement {
    sid    = "PermitDeliveryStatusMessagesToCloudWatchLogs"
    effect = "Allow"

    actions = [
      "logs:CreateLogGroup",
      "logs:CreateLogStream",
      "logs:PutLogEvents",
      "logs:PutMetricFilter",
      "logs:PutRetentionPolicy"
    ]

    resources = [
      aws_sns_topic.rdf_load.arn,
      aws_s3_bucket.source_data.arn
    ]
  }
}

