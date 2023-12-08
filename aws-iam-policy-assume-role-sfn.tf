data "aws_iam_policy_document" "assume_role_policy_for_sfn" {
  #provider = aws.target
  statement {
    actions = ["sts:AssumeRole"]
    sid     = ""
    effect  = "Allow"

    principals {
      type        = "Service"
      identifiers = [
        "states.amazonaws.com",
      ]
    }

    condition {
      test     = "ArnLike"
      values   = ["arn:aws:states:${var.aws_region}:${var.aws_account_id}:stateMachine:*"]
      variable = "aws:SourceArn"
    }

    condition {
      test     = "StringEquals"
      values   = [var.aws_account_id]
      variable = "aws:SourceAccount"
    }
  }
}
