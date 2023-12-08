import logging

from ...packages.aws_neptune.load_request import LoadRequest


def check_context(context, load_request: LoadRequest):
    logging.info("Invoked function ARN:" + context.invoked_function_arn)
    logging.info("CloudWatch log stream name:" + context.log_stream_name)
    logging.info("CloudWatch log group name:" + context.log_group_name)
    logging.info("Lambda Request ID:" + context.aws_request_id)
    logging.info("Lambda function memory limits in MB:" + context.memory_limit_in_mb)

    load_request.invoked_function_arn = context.invoked_function_arn
