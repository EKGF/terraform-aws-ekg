resource "aws_lambda_function" "invoke" {
  provider         = aws.ekg_api
  function_name    = local.lambda_invoke_name
  filename         = data.archive_file.invoke.output_path
  source_code_hash = data.archive_file.invoke.output_base64sha256
  #  filename         = local.lambda_invoke_zip
  #  source_code_hash = filebase64sha256(local.lambda_invoke_zip)
  role             = aws_iam_role.lfn_invoke.arn
  handler          = "bootstrap"
  runtime          = "provided.al2"
  architectures    = ["arm64"]
  timeout          = 1 * 60
  memory_size      = 128

  environment {
    variables = {
      //
      EKG_BASE_INTERNAL             = var.ekg_base_internal
      EKG_ID_BASE_INTERNAL          = var.ekg_id_base_internal
      EKG_GRAPH_BASE_INTERNAL       = var.ekg_graph_base_internal
      EKG_ONTOLOGY_BASE_INTERNAL    = var.ekg_ontology_base_internal
      //
      EKG_BASE_EXTERNAL             = var.ekg_base_external
      EKG_ID_BASE_EXTERNAL          = var.ekg_id_base_external
      EKG_GRAPH_BASE_EXTERNAL       = var.ekg_graph_base_external
      EKG_ONTOLOGY_BASE_EXTERNAL    = var.ekg_ontology_base_external
      //
      EKG_API_BASE                  = var.ekg_api_base
      //
      EKG_PIPELINE_ID               = var.name
      //
      neptune_s3_iam_role_arn       = var.neptune_s3_iam_role_arn
      neptune_s3_bucket_region      = var.aws_region
      //
      rdf_load_sfn_arn              = aws_sfn_state_machine.rdf_load.arn
      //
      AWS_NEPTUNE_LOAD_IAM_ROLE_ARN = var.neptune_s3_iam_role_arn
    }
  }

  vpc_config {
    subnet_ids         = var.neptune_cluster_subnet_ids
    security_group_ids = var.neptune_cluster_security_group_ids
  }

  depends_on = [
    aws_cloudwatch_log_group.lfn_invoke,
    null_resource.invoke
  ]

  tags = local.default_tags
}

resource "aws_lambda_permission" "allow_sns_invoke" {
  statement_id  = "AllowExecutionFromSNS"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.invoke.function_name
  principal     = "sns.amazonaws.com"
  source_arn    = aws_sns_topic.rdf_load.arn
}
