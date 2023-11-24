resource "aws_lambda_function" "rdf_load" {
  function_name    = "${local.prefix}-rdf-load"
  filename         = data.archive_file.artifact_zip.output_path
  source_code_hash = data.archive_file.artifact_zip.output_base64sha256
  role             = aws_iam_role.lambda_rdf_load.arn
  runtime          = "python3.11"
  handler          = "lambda_function.lambda_handler"
  timeout          = 10 * 60
  memory_size      = 128

  environment {
    variables = {
      neptune_endpoint         = var.sparql_update_endpoint_host
      neptune_port             = var.sparql_update_endpoint_port
      neptune_s3_iam_role_arn  = var.neptune_s3_iam_role_arn
      neptune_s3_bucket_region = var.aws_region
    }
  }

  vpc_config {
    subnet_ids         = var.neptune_cluster_subnet_ids
    security_group_ids = var.neptune_cluster_security_group_ids
  }

  depends_on = [
    aws_cloudwatch_log_group.lambda_log_group_rdf_load,
    #    data.external.execute_build,
    data.archive_file.artifact_zip,
  ]

  tags = local.default_tags
}

resource "aws_lambda_permission" "allow_sns_invoke" {
  statement_id  = "AllowExecutionFromSNS"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.rdf_load.function_name
  principal     = "sns.amazonaws.com"
  source_arn    = aws_sns_topic.rdf_load.arn
}
