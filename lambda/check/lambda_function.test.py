import json
import logging
import os
import unittest
from types import SimpleNamespace

import jsonpickle

# from aws_xray_sdk.core import xray_recorder

logger = logging.getLogger()
# xray_recorder.configure(
#   context_missing='LOG_ERROR'
# )

# xray_recorder.begin_segment('test_init')
function = __import__('lambda_function')
handler = function.lambda_handler


# xray_recorder.end_segment()

class TestFunction(unittest.TestCase):

    @staticmethod
    def get_event_and_context():
        with open('event.json', 'rb') as file:
            ba = bytearray(file.read())
            event = jsonpickle.decode(ba)
            logger.warning('## EVENT')
            logger.warning(jsonpickle.encode(event))

            context_data = """{
                "aws_request_id": "abcdef",
                "invoked_function_arn": "arn:aws:lambda:eu-west-1:123456789012:function:rdf-load",
                "log_group_name": "/aws/lambda/rdf-load",
                "log_stream_name": "2023/09/18/[$LATEST]cd48b64198ed4affb6464aff3d65f4cc",
                "memory_limit_in_mb": "128",
                "function_name": "rdf-load",
                "function_version": "$LATEST"
            }"""

            context = json.loads(context_data, object_hook=lambda d: SimpleNamespace(**d))

            os.environ[
                "neptune_endpoint"] = "ekgf-dt-dev-staging.cluster-ckvyb9qgnfdp.antartica-01.neptune.amazonaws.com"
            os.environ["neptune_port"] = "8182"
            os.environ["neptune_s3_iam_role_arn"] = "arn:aws:iam::123456789:role/ekgf-dt-dev-neptune"
            os.environ["AWS_LAMBDA_LOG_GROUP_NAME"] = context.log_group_name
            os.environ["AWS_LAMBDA_LOG_STREAM_NAME"] = context.log_stream_name
            os.environ["EKG_BASE_INTERNAL"] = "https://placeholder.kg"
            os.environ["EKG_ID_BASE_INTERNAL"] = "https://placeholder.kg/id"
            os.environ["EKG_GRAPH_BASE_INTERNAL"] = "https://placeholder.kg/graph"
            os.environ["EKG_ONTOLOGY_BASE_INTERNAL"] = "https://placeholder.kg/ontology"
            os.environ["EKG_BASE_EXTERNAL"] = "https://localhost:3000"
            os.environ["EKG_ID_BASE_EXTERNAL"] = "https://localhost:3000/id"
            os.environ["EKG_GRAPH_BASE_EXTERNAL"] = "https://localhost:3000/graph"
            os.environ["EKG_ONTOLOGY_BASE_EXTERNAL"] = "https://localhost:3000/ontology"
            os.environ["EKG_API_BASE"] = "https://localhost:3000/api"
            os.environ["EKG_SPARQL_LOADER_ENDPOINT"] = "https://localhost:7878"
            os.environ["EKG_SPARQL_HEALTH_ENDPOINT"] = "https://localhost:7878"
            os.environ["EKG_SPARQL_QUERY_ENDPOINT"] = "https://localhost:7878/sparql"
            os.environ["EKG_SPARQL_UPDATE_ENDPOINT"] = "https://localhost:7878/sparql"

            return event, context
        return None

    def test_simple(self):
        print('test_simple:')
        event, context = TestFunction.get_event_and_context()
        result = handler(event, context)
        print(str(result))
        self.assertRegex(str(result), 'FunctionCount', 'Should match')

    # def test_function(self):
    #   print('test_function:')
    #   xray_recorder.begin_segment('test_function')
    #   event, context = TestFunction.get_event_and_context()
    #   result = handler(event, context)
    #   print(str(result))
    #   self.assertRegex(str(result), 'FunctionCount', 'Should match')
    #   xray_recorder.end_segment()


if __name__ == '__main__':
    unittest.main()
