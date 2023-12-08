#
# Policy for the Step Function that checks whether the RDF load is complete
#
data "aws_iam_policy_document" "sfn" {

  statement {
    effect    = "Allow"
    actions   = ["lambda:InvokeFunction"]
    resources = [
      "arn:aws:lambda:${var.aws_region}:${var.aws_account_id}:function:${local.lambda_load_name}"
    ]
  }

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


