import logging
import os
import unittest

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
            os.environ[
                "neptune_endpoint"] = "ekgf-dt-dev-staging.cluster-ckvyb9qgnfdp.eu-west-2.neptune.amazonaws.com"
            os.environ["neptune_port"] = "8182"
            os.environ["neptune_s3_iam_role_arn"] = "arn:aws:iam::457852604093:role/ekgf-dt-dev-neptune"
            os.environ["AWS_LAMBDA_LOG_GROUP_NAME"] = "/aws/lambda/rdf-load"
            os.environ["AWS_LAMBDA_LOG_STREAM_NAME"] = "2023/09/18/[$LATEST]cd48b64198ed4affb6464aff3d65f4cc"
            context = {'requestid': '1234'}
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
