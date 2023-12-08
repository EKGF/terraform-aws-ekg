import logging

import requests

from packages.aws_neptune.endpoint import check_loader_endpoint
from packages.aws_neptune.error_code import convert_neptune_error_code
from packages.aws_neptune.neptune import convert_neptune_load_output
from . import loader_load_timeout


def send_upload_request(
        ekg_sparql_loader_endpoint: object,
        load_request: object
) -> object:
    # TODO: Check whether the loader endpoint == upload endpoint.
    # If so, use SPARQL LOAD instead of POSTing a Neptune-specific
    # load request to the loader endpoint so that we can use this
    # function for other triple-stores as well such as Oxigraph
    # or RDFox for local development or various different ingestion
    # pipelines for example.
    logging.info(f"## Neptune loader endpoint: {ekg_sparql_loader_endpoint}")

    result = check_loader_endpoint(ekg_sparql_loader_endpoint)
    if result is not None:
        return result

    headers = {'Content-Type': 'application/json'}

    try:
        logging.info(f"Posting load request, timeout={loader_load_timeout}")
        response = requests.post(
            ekg_sparql_loader_endpoint,
            json=load_request,
            headers=headers,
            timeout=loader_load_timeout
        )
        logging.debug(f'Response HTTP Status Code: {response.status_code}')
        response.raise_for_status()
        return convert_neptune_load_output(response.json())
    except requests.exceptions.Timeout:
        # back off and retry
        logging.error("Timeout occurred")
        return {
            "statusCode": 500,
            "statusError": "Timeout occurred"
        }
    except requests.exceptions.ConnectionError:
        logging.error("Connection error")
        return {
            "statusCode": 500,
            "statusError": "Connection error"
        }
    except ConnectionRefusedError:
        logging.error("Connection refused")
        return {
            "statusCode": 500,
            "statusError": "Connection refused"
        }
    except requests.exceptions.HTTPError as e:
        logging.error(f"HTTP Error occurred: {e}")
        error_json = response.json()
        logging.error(f"HTTP Error JSON: {error_json}")
        if response.status_code == 400 and error_json['code'] == "BadRequestException":
            # Response will be something like:
            # {
            #     "detailedMessage": "Failed to start new load for the source s3:///...somefile.ttl.
            #                         Max load task queue size limit breached. Limit is 64",
            #     "code": "BadRequestException",
            #     "requestId": "d0c61d09-6fed-01bd-feb4-a6fd25d1a72f",
            #     "message": "Failed to start new load for the source s3://..somefile.ttl.
            #                 Max load task queue size limit breached. Limit is 64"
            # }
            status_code_detail = convert_neptune_error_code(error_json)
            return {
                "statusCode": response.status_code,
                "statusCodeDetail": status_code_detail,
                "statusError": f"HTTP Error occurred: {e}, {error_json['message']}",
                "statusDetail": error_json
            }
        else:
            return {
                "statusCode": response.status_code,
                "statusError": f"HTTP Error occurred: {e}",
                "statusDetail": error_json
            }
    except requests.exceptions.RequestException as e:
        logging.error(f"Exception occurred: {e}")
        return {
            "statusCode": 500,
            "statusError": f"Exception occurred: {e}"
        }
