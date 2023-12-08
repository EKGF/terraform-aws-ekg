# Create the IAM role that the load lambda function will use
resource "aws_iam_role" "lfn_load" {
  #provider              = aws.target
  name                 = local.lfn_role_load
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

resource "aws_iam_role_policy" "lfn_load" {
  name   = local.lfn_role_load
  role   = aws_iam_role.lfn_load.id
  policy = data.aws_iam_policy_document.lfn_load.json
}