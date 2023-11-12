data "aws_iam_policy_document" "assume_role_policy_for_service" {
  #provider = aws.target
  statement {
    actions = ["sts:AssumeRole"]
    sid     = ""
    effect  = "Allow"

    principals {
      type        = "Service"
      identifiers = [
        "ec2.amazonaws.com",
        "lambda.amazonaws.com",
        "sns.amazonaws.com",
        "cloudtrail.amazonaws.com",
      ]
    }
  }
}
