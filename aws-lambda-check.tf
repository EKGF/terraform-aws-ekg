resource "aws_lambda_function" "check" {
  provider         = aws.ekg_api
  function_name    = local.lambda_check_name
  filename         = data.archive_file.check.output_path
  source_code_hash = data.archive_file.check.output_base64sha256
  role             = aws_iam_role.lfn_invoke.arn
  handler          = "bootstrap"
  runtime          = "provided.al2"
  architectures    = ["arm64"]
  timeout          = 1 * 60
  memory_size      = 128

  environment {
    variables = {
      //
      EKG_BASE_INTERNAL          = var.ekg_base_internal
      EKG_ID_BASE_INTERNAL       = var.ekg_id_base_internal
      EKG_GRAPH_BASE_INTERNAL    = var.ekg_graph_base_internal
      EKG_ONTOLOGY_BASE_INTERNAL = var.ekg_ontology_base_internal
      //
      EKG_BASE_EXTERNAL          = var.ekg_base_external
      EKG_ID_BASE_EXTERNAL       = var.ekg_id_base_external
      EKG_GRAPH_BASE_EXTERNAL    = var.ekg_graph_base_external
      EKG_ONTOLOGY_BASE_EXTERNAL = var.ekg_ontology_base_external
      //
      EKG_API_BASE               = var.ekg_api_base
      //
      EKG_SPARQL_LOADER_ENDPOINT = var.ekg_sparql_loader_endpoint
      EKG_SPARQL_HEALTH_ENDPOINT = var.ekg_sparql_health_endpoint
      EKG_SPARQL_QUERY_ENDPOINT  = var.ekg_sparql_query_endpoint
      EKG_SPARQL_UPDATE_ENDPOINT = var.ekg_sparql_update_endpoint
      //
      EKG_PIPELINE_ID            = var.name
      neptune_s3_iam_role_arn    = var.neptune_s3_iam_role_arn
      neptune_s3_bucket_region   = var.aws_region
    }
  }

  vpc_config {
    subnet_ids         = var.neptune_cluster_subnet_ids
    security_group_ids = var.neptune_cluster_security_group_ids
  }

  depends_on = [
    aws_cloudwatch_log_group.lfn_check,
    null_resource.load,
  ]

  tags = local.default_tags
}
