resource "aws_iam_role" "sns_feedback_role" {
  provider              = aws.ekg_api
  name                  = "${local.prefix}-sns-feedback"
  path                  = local.path
  force_detach_policies = true
  assume_role_policy    = data.aws_iam_policy_document.assume_role_policy_for_service.json
  permissions_boundary  = local.permissions_boundary
  tags                  = local.default_tags
}

resource "aws_iam_role_policy" "sns_feedback_policy" {
  name   = "${local.prefix}-sns-feedback-policy"
  role   = aws_iam_role.sns_feedback_role.id
  policy = data.aws_iam_policy_document.cloudwatch_writer.json
}
