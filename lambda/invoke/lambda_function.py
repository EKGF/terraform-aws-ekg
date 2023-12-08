import logging

from botocore.exceptions import ClientError

from check_context import check_context
from check_environment import check_environment
from check_event import check_event
from ...packages.aws_neptune.load_request import LoadRequest
from ...packages.aws_sfn.state_machine import StateMachine
from ...packages.aws_sfn.state_machine_invoke_request import StateMachineInvokeRequest
from ...packages.general.input_error import InputError


def lambda_handler(event, context):
    logging.basicConfig(level="DEBUG")
    root = logging.getLogger()
    root.setLevel(logging.DEBUG)
    logging.getLogger("urllib3").setLevel(logging.DEBUG)

    logging.info("## STARTING LAMBDA FUNCTION")

    load_request = LoadRequest()
    state_machine_invoke_request = StateMachineInvokeRequest(load_request)

    try:
        check_context(context, load_request)
        check_environment(state_machine_invoke_request)
        check_event(event, load_request)
        state_machine_invoke_request.validate()
    except InputError as e:
        logging.error(f"Input error: {e}")
        return e.json()

    logging.info(f"## Load request: {load_request}")
    # pprint.pprint(load_request)

    state_machine = StateMachine.create()
    try:
        execution_arn = state_machine.start(
            state_machine_arn=state_machine_invoke_request.rdf_load_sfn_arn,
            run_input=state_machine_invoke_request.load_request.neptune_load_request(),
        )
        logging.info(f"Started state machine {execution_arn}")

        return {}
    except ClientError as err:
        return {}

    # logging.info(f"## Result: {result}")
    # return result
