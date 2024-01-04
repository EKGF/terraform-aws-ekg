# Create the IAM role that the invoke lambda function will use
resource "aws_iam_role" "lfn_invoke" {
  provider             = aws.ekg_api
  name                 = local.lfn_role_invoke
  path                 = local.path
  assume_role_policy   = data.aws_iam_policy_document.assume_role_policy_for_service.json
  permissions_boundary = local.permissions_boundary
  tags                 = local.default_tags
  managed_policy_arns  = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  ]
}

resource "aws_iam_role_policy" "lfn_invoke" {
  name   = local.lfn_role_invoke
  role   = aws_iam_role.lfn_invoke.id
  policy = data.aws_iam_policy_document.lfn_invoke.json
}