import json

from ..aws_neptune.load_request import LoadRequest
from ..general.input_error import InputError


class StateMachineInvokeRequestError(InputError):
    """Exception raised for errors in the load request."""

    def __init__(self, var_name: str):
        self.status_text = f"State machine invoke request error: {var_name} not set"
        super().__init__(500, self.status_text)


class StateMachineInvokeRequest:
    def __init__(self, load_request: LoadRequest):
        self.load_request = load_request
        self.rdf_load_sfn_arn = None

    def __str__(self):
        return f"StateMachineInvokeRequest({self.json()})"

    def validate(self):
        if not self.rdf_load_sfn_arn:
            raise StateMachineInvokeRequestError("rdf_load_sfn_arn")
        self.load_request.validate()

    def json(self):
        return json.dumps(self, default=lambda o: o.__dict__, sort_keys=True, indent=4)
