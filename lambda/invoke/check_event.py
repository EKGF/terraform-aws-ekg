import json
import logging
import pathlib
import pprint

from constants import rdf_extension_format_map
from ...packages.aws_neptune.load_request import LoadRequest
from ...packages.general.input_error import InputError


def check_event(event, load_request: LoadRequest):
    logging.debug(f"## EVENT = {event}")
    # print("## EVENT")
    # pprint.pprint(event)
    # print("## CONTEXT")
    # pprint.pprint(context)

    event_records = event["Records"]

    if len(event_records) == 0:
        raise InputError(500, "No event_records in event")

    if len(event_records) > 1:
        raise InputError(500, "More than one record in event")

    event_record = event_records[0]
    if "Sns" not in event_record:
        raise InputError(500, "No Sns in event_record")

    sns_event = event_record["Sns"]
    if "Message" not in sns_event:
        raise InputError(500, "No Message in sns_event")

    sns_message = sns_event["Message"]
    if "Records" not in sns_message:
        raise InputError(500, "No Records in sns_message")

    logging.debug(f"## sns_message as string = {sns_message}")
    # Since SNS message comes as embedded json, we need to parse it to an object first
    sns_message = json.loads(sns_message)
    logging.debug(f"## sns_message = {sns_message}")
    sns_event_records = sns_message["Records"]

    if len(sns_event_records) == 0:
        raise InputError(500, "No sns_event_records in sns_message")

    if len(sns_event_records) > 1:
        raise InputError(500, "More than one record in sns_message")

    sns_message_record = sns_event_records[0]
    logging.debug(f"## sns_message_record = {sns_message_record}")
    # Example of an event (content of sns_message_record at this point):
    # {
    #     "eventVersion": "2.1",
    #     "eventSource": "aws:s3",
    #     "awsRegion": "antartica-01",
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
    logging.debug(f"## Event {sns_message_record['eventSource']}:{s3_event_name}")
    s3_info = sns_message_record["s3"]
    s3_bucket = s3_info["bucket"]["name"]
    logging.debug(f"## S3 bucket: {s3_bucket}")
    s3_bucket_region = sns_message_record["awsRegion"]
    logging.debug(f"## S3 bucket region: {s3_bucket_region}")
    load_request.s3_bucket_region = s3_bucket_region
    s3_object = s3_info["object"]
    logging.debug(f"## S3 object: {s3_object}")
    pprint.pprint(s3_object)
    s3_key = s3_object["key"]
    logging.info(f"## S3 key: {s3_key}")
    if "size" in s3_object:
        s3_size = s3_object["size"]
        logging.info(f"## S3 size: {s3_size}")
    s3_uri = f"s3://{s3_bucket}/{s3_key}"  # something like s3://bucket-name/object-key-name
    logging.info(f"### S3 uri: {s3_uri}")
    load_request.s3_uri = s3_uri

    s3_ext = pathlib.Path(s3_key).suffix
    if s3_ext in rdf_extension_format_map:
        rdf_format = rdf_extension_format_map[s3_ext]
        logging.info(f"### RDF format: {rdf_format}")
        load_request.rdf_format = rdf_format
    else:
        raise InputError(500, f"Unsupported RDF extension {s3_ext}")
