import logging
import os

from ...packages.aws_sfn.state_machine_invoke_request import StateMachineInvokeRequest
from ...packages.general.input_error import InputError


def mandatory_env_var(name: str):
    if name not in os.environ:
        raise InputError(500, f"Environment variable {name} not set")
    return os.environ.get(name)


def check_environment(state_machine_invoke_request: StateMachineInvokeRequest):
    aws_lambda_log_group_name = mandatory_env_var("AWS_LAMBDA_LOG_GROUP_NAME")
    aws_lambda_log_stream_name = mandatory_env_var("AWS_LAMBDA_LOG_GROUP_NAME")

    neptune_s3_iam_role_arn = mandatory_env_var("neptune_s3_iam_role_arn")

    rdf_load_sfn_arn = mandatory_env_var("rdf_load_sfn_arn")

    ekg_base_internal = mandatory_env_var("EKG_BASE_INTERNAL")
    ekg_id_base_internal = mandatory_env_var("EKG_ID_BASE_INTERNAL")
    ekg_graph_base_internal = mandatory_env_var("EKG_GRAPH_BASE_INTERNAL")
    ekg_ontology_base_internal = mandatory_env_var("EKG_ONTOLOGY_BASE_INTERNAL")

    ekg_base_external = mandatory_env_var("EKG_BASE_EXTERNAL")
    ekg_id_base_external = mandatory_env_var("EKG_ID_BASE_EXTERNAL")
    ekg_graph_base_external = mandatory_env_var("EKG_GRAPH_BASE_EXTERNAL")
    ekg_ontology_base_external = mandatory_env_var("EKG_ONTOLOGY_BASE_EXTERNAL")

    ekg_api_base = mandatory_env_var("EKG_API_BASE")

    ekg_sparql_loader_endpoint = mandatory_env_var("EKG_SPARQL_LOADER_ENDPOINT")
    ekg_sparql_health_endpoint = mandatory_env_var("EKG_SPARQL_HEALTH_ENDPOINT")
    ekg_sparql_query_endpoint = mandatory_env_var("EKG_SPARQL_QUERY_ENDPOINT")
    ekg_sparql_update_endpoint = mandatory_env_var("EKG_SPARQL_UPDATE_ENDPOINT")

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

    state_machine_invoke_request.rdf_load_sfn_arn = rdf_load_sfn_arn
    state_machine_invoke_request.load_request.neptune_s3_iam_role_arn = neptune_s3_iam_role_arn
    state_machine_invoke_request.load_request.rdf_load_sfn_arn = rdf_load_sfn_arn
    state_machine_invoke_request.load_request.ekg_id_base_internal = ekg_id_base_internal
