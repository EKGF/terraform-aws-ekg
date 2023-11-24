locals {

  stack     = "${var.org_short}-${var.project_short}-${var.environment}"
  stack_ci  = "${var.org_short}-${var.project_short}-${var.environment}-ci"
  path      = "/${var.org_short}-${var.project_short}/${var.environment}/"
  path_ci   = "/${var.org_short}-${var.project_short}/${var.environment}/ci/"
  prefix    = "${local.stack}-${var.name}"
  full_name = "${local.prefix}-rdf-load"

  default_tags = {
    org_short   = var.org_short
    project     = var.project_short
    environment = var.environment
    vpc         = var.vpc_name
  }

  permissions_boundary = var.iam_permissions_boundary == null ? null : "arn:aws:iam::${var.aws_account_id}:policy/${var.iam_permissions_boundary}"

  rdf_load_path = "${path.module}/lambda/rdf-load"
  python_package_path = "${local.rdf_load_path}/.package"
  artifact_zip  = "${local.rdf_load_path}/artifact.zip"
}
