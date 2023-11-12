import json
import logging
import os
import pathlib
import pprint
import socket
from contextlib import closing

import requests

rdf_extension_format_map = {
    ".ttl": "turtle",
    ".nt": "ntriples"
}


def check_neptune_endpoint(host_name, port):
    print(f"Checking Neptune endpoint: {host_name}:{str(port)}")
    machine_ip = socket.gethostbyname(host_name)
    print(f"Neptune endpoint IP Address: {machine_ip}")
    with closing(socket.socket(socket.AF_INET, socket.SOCK_STREAM)) as sock:
        print(f"Connecting to Neptune endpoint {host_name}:{str(port)}")
        sock.settimeout(3)
        print("With timeout set to 3 seconds")
        if sock.connect_ex((machine_ip, port)) == 0:
            print(f"Neptune endpoint {host_name}:{str(port)} is open")
            return None
        else:
            print(f"ERROR: Neptune endpoint {host_name}:{str(port)} is not open")
            return {
                "statusCode": 500,
                "statusError": f"Neptune endpoint {host_name}:{str(port)} is not open"
            }


def send_upload_reguest(neptune_staging_endpoint, neptune_port, load_request):
    url = f"https://{neptune_staging_endpoint}:{str(neptune_port)}/loader"

    print(f"## Neptune endpoint: {url}")

    result = check_neptune_endpoint(neptune_staging_endpoint, int(neptune_port))
    if result is not None:
        return result

    headers = {'Content-Type': 'application/json'}

    try:
        timeout = 30
        print(f"Posting load request, timeout={timeout}")
        response = requests.post(url, json=load_request, headers=headers, timeout=timeout)
        response.raise_for_status()
        return response
    except requests.exceptions.Timeout:
        # back off and retry
        print("Timeout occurred")
        return {
            "statusCode": 500,
            "statusError": "Timeout occurred"
        }
    except requests.exceptions.ConnectionError:
        print("Connection error")
        return {
            "statusCode": 500,
            "statusError": "Connection error"
        }
    except ConnectionRefusedError:
        print("Connection refused")
        return {
            "statusCode": 500,
            "statusError": "Connection refused"
        }
    except requests.exceptions.HTTPError as e:
        print(f"HTTP Error occurred: {e}")
        return {
            "statusCode": response.status_code,
            "statusError": f"HTTP Error occurred: {e}"
        }
    except requests.exceptions.RequestException as e:
        print(f"Exception occurred: {e}")
        return {
            "statusCode": 500,
            "statusError": f"Exception occurred: {e}"
        }


def lambda_handler(event, context):
    logging.basicConfig(level="DEBUG")
    root = logging.getLogger()
    root.setLevel(logging.DEBUG)
    logging.getLogger("urllib3").setLevel(logging.DEBUG)

    print("## STARTING LAMBDA FUNCTION")
    print("Invoked function ARN:" + context.invoked_function_arn)
    print("CloudWatch log stream name:" + context.log_stream_name)
    print("CloudWatch log group name:" + context.log_group_name)
    print("Lambda Request ID:" + context.aws_request_id)
    print("Lambda function memory limits in MB:" + context.memory_limit_in_mb)

    aws_lambda_log_group_name = os.environ["AWS_LAMBDA_LOG_GROUP_NAME"]
    aws_lambda_log_stream_name = os.environ["AWS_LAMBDA_LOG_GROUP_NAME"]
    neptune_staging_endpoint = os.environ["neptune_staging_endpoint"]
    neptune_port = os.environ["neptune_port"]
    neptune_s3_iam_role_arn = os.environ["neptune_s3_iam_role_arn"]

    # TODO: Add error handling

    print("## ENVIRONMENT VARIABLES")
    print(f"AWS_LAMBDA_LOG_GROUP_NAME  = {aws_lambda_log_group_name}")
    print(f"AWS_LAMBDA_LOG_STREAM_NAME = {aws_lambda_log_stream_name}")
    print(f"neptune_staging_endpoint   = {neptune_staging_endpoint}")
    print(f"neptune_port               = {neptune_port}")
    print(f"neptune_s3_iam_role_arn    = {neptune_s3_iam_role_arn}")
    print(f"## EVENT = {event}")
    # print("## EVENT")
    # pprint.pprint(event)
    # print("## CONTEXT")
    # pprint.pprint(context)

    event_records = event["Records"]
    if len(event_records) == 0:
        print("ERROR: No event_records in event")
        return {
            "statusCode": 500,
        }
    if (len(event_records) > 1):
        print("ERROR: More than one record in event")
        return {
            "statusCode": 500,
        }

    event_record = event_records[0]
    if "Sns" not in event_record:
        print("ERROR: No Sns in event_record")
        return {
            "statusCode": 500,
        }
    sns_event = event_record["Sns"]
    if "Message" not in sns_event:
        print("ERROR: No Message in sns_event")
        return {
            "statusCode": 500,
        }
    sns_message = sns_event["Message"]
    if "Records" not in sns_message:
        print("ERROR: No Records in sns_message")
        return {
            "statusCode": 500,
        }
    print(f"## sns_message as string = {sns_message}")
    # Since SNS message comes as embedded json, we need to parse it to an object first
    sns_message = json.loads(sns_message)
    print(f"## sns_message = {sns_message}")
    sns_event_records = sns_message["Records"]
    if len(sns_event_records) == 0:
        print("ERROR: No sns_event_records in sns_message")
        return {
            "statusCode": 500,
        }
    if (len(sns_event_records) > 1):
        print("ERROR: More than one record in sns_message")
        return {
            "statusCode": 500,
        }
    sns_message_record = sns_event_records[0]
    print(f"## sns_message_record = {sns_message_record}")
    # Example of an event (content of sns_message_record at this point):
    # {
    #     "eventVersion": "2.1",
    #     "eventSource": "aws:s3",
    #     "awsRegion": "eu-west-2",
    #     "eventTime": "2023-09-18T10:03:15.979Z",
    #     "eventName": "ObjectCreated:Put",
    #     "userIdentity": {
    #         "principalId": "AWS:AIDAWVGREJ265Q72HOJUP"
    #     },
    #     "requestParameters": {
    #         "sourceIPAddress": "193.237.90.75"
    #     },
    #     "responseElements": {
    #         "x-amz-request-id": "JJ807NMA5B2VMJ0D",
    #         "x-amz-id-2": "wSZ0gf3XaMj63uKcY7A43KSJ3fAMm27hZcWZQRTNzhFIq4oaTZ7fO1RaIL35VbG3g9LIU/B6+IuDLN1N1lAoeJapphdeOaTu"
    #     },
    #     "s3": {
    #         "s3SchemaVersion": "1.0",
    #         "configurationId": "tf-s3-topic-20230915095940816500000001",
    #         "bucket": {
    #             "name": "ekgf-dt-dev-metadata",
    #             "ownerIdentity": {
    #                 "principalId": "A1M8OTUP4LUCQC"
    #             },
    #             "arn": "arn:aws:s3:::ekgf-dt-dev-metadata"
    #         },
    #         "object": {
    #             "key": "static-dataset/personas/ekgf-group-internal-auditor.ttl",
    #             "size": 1206,
    #             "eTag": "455c556f7d1b7f8587ecabe2dd8184af",
    #             "versionId": "LBK4atYjFZR7h5v_.bUVAuWLbYpwCeB2",
    #             "sequencer": "0065082063F0F5766D"
    #         }
    #     }
    # }
    s3_event_name = sns_message_record['eventName']
    print(f"## Event {sns_message_record['eventSource']}:{s3_event_name}")
    s3_info = sns_message_record["s3"]
    s3_bucket = s3_info["bucket"]["name"]
    print(f"## S3 bucket: {s3_bucket}")
    s3_bucket_region = sns_message_record["awsRegion"]
    print(f"## S3 bucket region: {s3_bucket_region}")
    s3_object = s3_info["object"]
    print(f"## S3 object: {s3_object}")
    pprint.pprint(s3_object)
    s3_key = s3_object["key"]
    print(f"## S3 key: {s3_key}")
    if "size" in s3_object:
        s3_size = s3_object["size"]
        print(f"## S3 size: {s3_size}")
    s3_uri = f"s3://{s3_bucket}/{s3_key}"  # something like s3://bucket-name/object-key-name
    print(f"### S3 uri: {s3_uri}")

    s3_ext = pathlib.Path(s3_key).suffix
    if s3_ext in rdf_extension_format_map:
        rdf_format = rdf_extension_format_map[s3_ext]
        print(f"### RDF format: {rdf_format}")
    else:
        print(f"WARNING: Unsupported RDF extension {s3_ext}")
        return {'status': 500}

    #
    # See https://docs.aws.amazon.com/neptune/latest/userguide/load-api-reference-load.html
    #
    load_request = {
        "source": s3_uri,
        "format": rdf_format,
        "iamRoleArn": neptune_s3_iam_role_arn,
        "mode": "NEW",
        "region": s3_bucket_region,
        "failOnError": "TRUE",
        "parallelism": "HIGH",
        "parserConfiguration": {
            "baseUri": "https://ekgf.org/id/",  # TODO: Make configurable
            # "namedGraphUri" : "http://ekgf.org/graph/all-data" # TODO: Change to dataset name
            "namedGraphUri": "http://aws.amazon.com/neptune/vocab/v01/DefaultNamedGraph"  # TODO: Change to dataset name
        },
        "updateSingleCardinalityProperties": "FALSE",
        "queueRequest": "TRUE",
        "dependencies": []
    }

    print(f"## Load request: {load_request}")
    # pprint.pprint(load_request)

    result = send_upload_reguest(neptune_staging_endpoint, neptune_port, load_request)
    print(f"## Result: {result}")
    return result
