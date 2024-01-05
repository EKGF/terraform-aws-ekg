locals {

  stack           = "${var.org_short}-${var.project_short}-${var.environment}"
  stack_ci        = "${var.org_short}-${var.project_short}-${var.environment}-ci"
  path            = "/${var.org_short}-${var.project_short}/${var.environment}/"
  path_ci         = "/${var.org_short}-${var.project_short}/${var.environment}/ci/"
  prefix          = "${local.stack}-${var.name}"
  full_name       = "${local.stack}-${var.name}-loader"
  sns_topic       = "${local.stack}-${var.name}-new-rdf"
  sfn_role_name   = "${local.full_name}-sfn"
  sfn_policy_name = local.sfn_role_name
  lfn_role_invoke = "${local.full_name}-lfn-invoke"
  lfn_role_load   = "${local.full_name}-lfn-load"
  lfn_role_check  = "${local.full_name}-lfn-check"

  default_tags = {
    org_short   = var.org_short
    project     = var.project_short
    environment = var.environment
    vpc         = var.vpc_name
  }

  permissions_boundary = var.iam_permissions_boundary == null ? null : "arn:aws:iam::${var.aws_account_id}:policy/${var.iam_permissions_boundary}"

  // The lambda function "invoke" which is used to kick of the state machine
  lambda_invoke_name         = "${local.full_name}-invoke"
  lambda_invoke_crate        = "ekg-lfn-invoke"
  lambda_invoke_crate_path   = "${path.module}/crate/${local.lambda_invoke_crate}"
  lambda_invoke_package_path = "${path.module}/target/lambda/${local.lambda_invoke_crate}"
  lambda_invoke_zip          = "${local.lambda_invoke_package_path}/../tf-artifact-${local.lambda_invoke_crate}-${var.name}.zip"

  // The lambda function "load" which is used to instruct the Neptune loader to load an S3-based RDF file
  lambda_load_name         = "${local.full_name}-load"
  lambda_load_crate        = "ekg-lfn-load"
  lambda_load_crate_path   = "${path.module}/crate/${local.lambda_load_crate}"
  lambda_load_package_path = "${path.module}/target/lambda/${local.lambda_load_crate}"
  lambda_load_zip          = "${local.lambda_invoke_package_path}/../tf-artifact-${local.lambda_load_crate}-${var.name}.zip"

  // The lambda function "check" which is used to check the status of the Neptune load job
  lambda_check_name         = "${local.full_name}-check"
  lambda_check_crate        = "ekg-lfn-check"
  lambda_check_crate_path   = "${path.module}/crate/${local.lambda_check_crate}"
  lambda_check_package_path = "${path.module}/target/lambda/${local.lambda_check_crate}"
  lambda_check_zip          = "${local.lambda_check_package_path}/../tf-artifact-${local.lambda_check_crate}-${var.name}.zip"
}
