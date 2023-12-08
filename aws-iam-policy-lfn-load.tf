#
# Policy for the Lambda Function that kicks off the S3-file load by Neptune
#
data "aws_iam_policy_document" "lfn_load" {

  // TODO: Move the Neptune specific stuff here
  
  statement {
    effect  = "Allow"
    actions = [
      "logs:CreateLogDelivery",
      "logs:CreateLogStream",
      "logs:GetLogDelivery",
      "logs:UpdateLogDelivery",
      "logs:DeleteLogDelivery",
      "logs:ListLogDeliveries",
      "logs:PutLogEvents",
      "logs:PutResourcePolicy",
      "logs:DescribeResourcePolicies",
      "logs:DescribeLogGroups"
    ]
    resources = ["*"] // TODO: restrict to the log group
  }
}


