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

  lambda_load_name         = "${local.full_name}-load"
  lambda_load_path         = "${path.module}/lambda/load"
  lambda_load_package_path = "${local.lambda_load_path}/.package"
  lambda_load_zip          = "${local.lambda_load_path}/artifact-load-${var.name}.zip"

  lambda_invoke_name         = "${local.full_name}-invoke"
  lambda_invoke_path         = "${path.module}/lambda/invoke"
  lambda_invoke_package_path = "${local.lambda_invoke_path}/.package"
  lambda_invoke_zip          = "${local.lambda_invoke_path}/artifact-invoke-${var.name}.zip"

  lambda_check_name         = "${local.full_name}-check"
  lambda_check_path         = "${path.module}/lambda/check"
  lambda_check_package_path = "${local.lambda_check_path}/.package"
  lambda_check_zip          = "${local.lambda_check_path}/artifact-check-${var.name}.zip"
}
