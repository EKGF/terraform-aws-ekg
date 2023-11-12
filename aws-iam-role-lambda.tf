# Create the IAM role that the lambda functions will use
resource "aws_iam_role" "lambda_rdf_load" {
  #provider              = aws.target
  name                 = local.full_name
  path                 = local.path
  assume_role_policy   = data.aws_iam_policy_document.assume_role_policy_for_service.json
  permissions_boundary = local.permissions_boundary
  tags                 = local.default_tags
  managed_policy_arns  = [
    "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
    "arn:aws:iam::aws:policy/NeptuneFullAccess", # TODO: trim this down
    "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  ]
}
