data "aws_iam_policy_document" "assume_role_policy_for_service" {
  #provider = aws.target
  statement {
    actions = ["sts:AssumeRole"]
    sid     = ""
    effect  = "Allow"

    principals {
      type        = "Service"
      identifiers = [
        "ec2.amazonaws.com",
        "lambda.amazonaws.com",
        "sns.amazonaws.com",
        "cloudtrail.amazonaws.com",
        "states.amazonaws.com",
      ]
    }
  }
}

#  {
#  "Version":"2012-10-17",
#  "Statement":[
#     {
#        "Effect":"Allow",
#        "Principal":{
#           "Service":[
#              "states.amazonaws.com"
#           ]
#        },
#        "Action":"sts:AssumeRole",
#        "Condition":{
#           "ArnLike":{
#              "aws:SourceArn":"arn:aws:states:us-east-1:111122223333:stateMachine:*"
#           },
#           "StringEquals":{
#              "aws:SourceAccount":"111122223333"
#           }
#        }
#     }
#  ]
#}