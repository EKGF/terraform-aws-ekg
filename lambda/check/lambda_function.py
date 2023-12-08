import logging
import os

import packages.aws_neptune.load

rdf_extension_format_map = {
    ".ttl": "turtle",
    ".nt": "ntriples"
}


def lambda_handler(event, context):
    logging.basicConfig(level="DEBUG")
    root = logging.getLogger()
    root.setLevel(logging.DEBUG)
    logging.getLogger("urllib3").setLevel(logging.DEBUG)

    logging.info("## STARTING LAMBDA FUNCTION")
    logging.info("Invoked function ARN:" + context.invoked_function_arn)
    logging.info("CloudWatch log stream name:" + context.log_stream_name)
    logging.info("CloudWatch log group name:" + context.log_group_name)
    logging.info("Lambda Request ID:" + context.aws_request_id)
    logging.info("Lambda function memory limits in MB:" + context.memory_limit_in_mb)

    aws_lambda_log_group_name = os.environ["AWS_LAMBDA_LOG_GROUP_NAME"]
    aws_lambda_log_stream_name = os.environ["AWS_LAMBDA_LOG_GROUP_NAME"]
    neptune_s3_iam_role_arn = os.environ["neptune_s3_iam_role_arn"]

    ekg_base_internal = os.environ["EKG_BASE_INTERNAL"]
    ekg_id_base_internal = os.environ["EKG_ID_BASE_INTERNAL"]
    ekg_graph_base_internal = os.environ["EKG_GRAPH_BASE_INTERNAL"]
    ekg_ontology_base_internal = os.environ["EKG_ONTOLOGY_BASE_INTERNAL"]

    ekg_base_external = os.environ["EKG_BASE_EXTERNAL"]
    ekg_id_base_external = os.environ["EKG_ID_BASE_EXTERNAL"]
    ekg_graph_base_external = os.environ["EKG_GRAPH_BASE_EXTERNAL"]
    ekg_ontology_base_external = os.environ["EKG_ONTOLOGY_BASE_EXTERNAL"]

    ekg_api_base = os.environ["EKG_API_BASE"]

    ekg_sparql_loader_endpoint = os.environ["EKG_SPARQL_LOADER_ENDPOINT"]
    ekg_sparql_health_endpoint = os.environ["EKG_SPARQL_HEALTH_ENDPOINT"]
    ekg_sparql_query_endpoint = os.environ["EKG_SPARQL_QUERY_ENDPOINT"]
    ekg_sparql_update_endpoint = os.environ["EKG_SPARQL_UPDATE_ENDPOINT"]

    # TODO: Add error handling

    logging.info("## ENVIRONMENT VARIABLES")
    logging.info(f"AWS_LAMBDA_LOG_GROUP_NAME  = {aws_lambda_log_group_name}")
    logging.info(f"AWS_LAMBDA_LOG_STREAM_NAME = {aws_lambda_log_stream_name}")
    logging.info(f"neptune_s3_iam_role_arn    = {neptune_s3_iam_role_arn}")
    #
    logging.info(f"EKG_BASE_INTERNAL          = {ekg_base_internal}")
    logging.info(f"EKG_ID_BASE_INTERNAL       = {ekg_id_base_internal}")
    logging.info(f"EKG_GRAPH_BASE_INTERNAL    = {ekg_graph_base_internal}")
    logging.info(f"EKG_ONTOLOGY_BASE_INTERNAL = {ekg_ontology_base_internal}")
    #
    logging.info(f"EKG_BASE_EXTERNAL          = {ekg_base_external}")
    logging.info(f"EKG_ID_BASE_EXTERNAL       = {ekg_id_base_external}")
    logging.info(f"EKG_GRAPH_BASE_EXTERNAL    = {ekg_graph_base_external}")
    logging.info(f"EKG_ONTOLOGY_BASE_EXTERNAL = {ekg_ontology_base_external}")
    #
    logging.info(f"EKG_API_BASE               = {ekg_api_base}")
    #
    logging.info(f"EKG_SPARQL_LOADER_ENDPOINT = {ekg_sparql_loader_endpoint}")
    logging.info(f"EKG_SPARQL_HEALTH_ENDPOINT = {ekg_sparql_health_endpoint}")
    logging.info(f"EKG_SPARQL_QUERY_ENDPOINT  = {ekg_sparql_query_endpoint}")
    logging.info(f"EKG_SPARQL_UPDATE_ENDPOINT = {ekg_sparql_update_endpoint}")

    logging.info(f"## EVENT = {event}")
    # print("## EVENT")
    # pprint.pprint(event)
    # print("## CONTEXT")
    # pprint.pprint(context)

    if 'LoadOutput' in event:
        del event['LoadOutput']

    result = packages.aws_neptune.load.send_upload_request(ekg_sparql_loader_endpoint, event)
    logging.info(f"## Result: {result}")
    return result
