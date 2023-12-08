resource "aws_cloudwatch_log_group" "sns_topic_rdf_load" {
  count             = var.enable_sns_cloudwatch ? 1 : 0
  name              = "sns/${var.aws_region}/${var.aws_account_id}/${local.full_name}"
  skip_destroy      = true
  retention_in_days = 3
  tags              = local.default_tags
}

resource "aws_cloudwatch_log_group" "lfn_invoke" {
  name              = "/aws/lambda/${local.lambda_invoke_name}"
  skip_destroy      = true
  retention_in_days = 3
  tags              = local.default_tags
}

resource "aws_cloudwatch_log_group" "lfn_load" {
  name              = "/aws/lambda/${local.lambda_load_name}"
  skip_destroy      = true
  retention_in_days = 3
  tags              = local.default_tags
}

resource "aws_cloudwatch_log_group" "lfn_check" {
  name              = "/aws/lambda/${local.lambda_check_name}"
  skip_destroy      = true
  retention_in_days = 3
  tags              = local.default_tags
}

resource "aws_cloudwatch_log_group" "sfn" {
  name              = local.sfn_role_name
  skip_destroy      = true
  retention_in_days = 3
  tags              = local.default_tags
}
