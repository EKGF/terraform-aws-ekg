resource "aws_cloudwatch_log_group" "log_group_sns_topic_rdf_load" {
  count             = var.enable_sns_cloudwatch ? 1 : 0
  name              = "sns/${var.aws_region}/${var.aws_account_id}/${local.full_name}"
  skip_destroy      = true
  retention_in_days = 3
  tags              = local.default_tags
}

resource "aws_cloudwatch_log_group" "lambda_log_group_rdf_load" {
  name              = "/aws/lambda/${local.prefix}-rdf-load"
  skip_destroy      = true
  retention_in_days = 3
  tags              = local.default_tags
}
