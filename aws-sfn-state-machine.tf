resource "aws_sfn_state_machine" "rdf_load" {
  provider = aws.ekg_api
  name     = local.full_name
  role_arn = aws_iam_role.sfn_rdf_load.arn
  publish  = true

  definition = <<-EOF
  {
      "Comment": "The RDF Loader State Machine, invoked by the lambda function ${local.lambda_invoke_name}",
      "StartAt": "InstructNeptuneToLoad",
      "States": {
          "InstructNeptuneToLoad": {
              "Type": "Task",
              "Comment": "Instruct the Neptune bulk loader to load the given S3 file",
              "Resource": "${aws_lambda_function.load.arn}",
              "InputPath": "$",
              "TimeoutSeconds": 30,
              "ResultPath": "$.LoadOutput",
              "Next": "CheckIfInstructionGiven"
          },
          "CheckIfInstructionGiven": {
              "Type": "Choice",
              "Comment": "Check if the Neptune bulk loader was instructed successfully to load the given S3 file",
              "Choices": [
                  {
                      "Variable": "$.LoadOutput.statusCode",
                      "NumericEquals": 200,
                      "Next": "CheckIfLoaderFinished"
                  },
                  {
                      "Variable": "$.LoadOutput.detailError",
                      "StringEquals": "MaxLoadTaskQueueSizeLimitBreached",
                      "Next": "RetryLoadInstruction"
                  },
                  {
                      "Variable": "$.LoadOutput.detailError",
                      "StringEquals": "MaxConcurrentLoadLimitBreached",
                      "Next": "RetryLoadInstruction"
                  }
              ],
              "Default": "LoadInstructionFailed"
          },
          "RetryLoadInstruction": {
              "Type": "Wait",
              "Comment": "Wait 10 seconds and then retry the instruction to the Neptune bulk loader",
              "Seconds": 10,
              "Next": "InstructNeptuneToLoad"
          },
          "CheckIfLoaderFinished": {
              "Type": "Task",
              "Comment": "Check if the Neptune bulk loader has finished loading the given S3 file",
              "Resource": "${aws_lambda_function.check.arn}",
              "InputPath": "$",
              "TimeoutSeconds": 5,
              "ResultPath": "$.CheckOutput",
              "Next": "CheckIfLoaderFinishedSucceeded"
          },
          "CheckIfLoaderFinishedSucceeded": {
              "Type": "Succeed"
          },
          "LoadInstructionFailed": {
              "Type": "Fail"
          }
      }
  }
  EOF

  logging_configuration {
    log_destination        = "${aws_cloudwatch_log_group.sfn.arn}:*"
    include_execution_data = true
    level                  = "ALL"
  }
}


