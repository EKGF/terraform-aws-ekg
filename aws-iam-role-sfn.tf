# Create the IAM role that the step function will use
resource "aws_iam_role" "sfn_rdf_load" {
  provider             = aws.ekg_api
  name                 = local.sfn_role_name
  path                 = local.path
  assume_role_policy   = data.aws_iam_policy_document.assume_role_policy_for_sfn.json
  permissions_boundary = local.permissions_boundary
  tags                 = local.default_tags
  managed_policy_arns  = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/NeptuneFullAccess", # TODO: trim this down
    "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  ]
}

resource "aws_iam_role_policy" "step_function_rdf_load" {
  name   = local.sfn_role_name
  role   = aws_iam_role.sfn_rdf_load.id
  policy = data.aws_iam_policy_document.sfn.json
}