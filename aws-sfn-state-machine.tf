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
                      "Next": "CheckLoaderJobStatus"
                  },
                  {
                      "Variable": "$.LoadOutput.detailStatus",
                      "StringEquals": "Timedout",
                      "Next": "RetryLoadInstruction"
                  },
                  {
                      "Variable": "$.LoadOutput.detailStatus",
                      "StringEquals": "MaxLoadTaskQueueSizeLimitBreached",
                      "Next": "RetryLoadInstruction"
                  },
                  {
                      "Variable": "$.LoadOutput.detailStatus",
                      "StringEquals": "MaxConcurrentLoadLimitBreached",
                      "Next": "RetryLoadInstruction"
                  }
              ],
              "Default": "LoadInstructionFailed"
          },
          "RetryLoadInstruction": {
              "Type": "Wait",
              "Comment": "Wait a random number of seconds, as suggested by the load lambda function, and then retry the instruction to the Neptune bulk loader",
              "SecondsPath": "$.LoadOutput.suggestedRetrySeconds",
              "Next": "InstructNeptuneToLoad"
          },
          "CheckLoaderJobStatus": {
              "Type": "Task",
              "Comment": "Check if the Neptune bulk loader has finished loading the given S3 file",
              "Resource": "${aws_lambda_function.check.arn}",
              "InputPath": "$",
              "TimeoutSeconds": 5,
              "ResultPath": "$.CheckOutput",
              "Next": "CheckIfLoaderJobFinished"
          },
          "CheckIfLoaderJobFinished": {
              "Type": "Choice",
              "Comment": "Check if the Neptune bulk loader has finished the job successfully, failed or is still running",
              "Choices": [
                  {
                      "Variable": "$.CheckOutput.detailStatus",
                      "StringEquals": "LoaderJobCompleted",
                      "Next": "LoaderJobCompleted"
                  },
                  {
                      "Variable": "$.CheckOutput.detailStatus",
                      "StringEquals": "LoaderJobInQueue",
                      "Next": "RetryCheck"
                  },
                  {
                      "Variable": "$.CheckOutput.detailStatus",
                      "StringEquals": "LoaderJobNotStarted",
                      "Next": "RetryCheck"
                  },
                  {
                      "Variable": "$.CheckOutput.detailStatus",
                      "StringEquals": "LoaderJobInProgress",
                      "Next": "RetryCheck"
                  }
              ],
              "Default": "LoaderJobFailed"
          },
          "RetryCheck": {
              "Type": "Wait",
              "Comment": "Wait a random number of seconds, as suggested by the check lambda function, and then retry to get the latest status of the Neptune bulk loader",
              "SecondsPath": "$.CheckOutput.suggestedRetrySeconds",
              "Next": "CheckLoaderJobStatus"
          },
          "LoaderJobCompleted": {
              "Type": "Succeed"
          },
          "LoadInstructionFailed": {
              "Type": "Fail"
          },
          "LoaderJobFailed": {
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


