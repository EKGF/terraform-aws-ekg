import json

from ..general.input_error import InputError


class LoaderGetStatusRequestError(InputError):
    """Exception raised for errors in the loader Get-Status request."""

    def __init__(self, var_name: str):
        self.status_text = f"Loader Get-Status request error: {var_name} not set"
        super().__init__(500, self.status_text)


class LoaderGetStatusRequest:
    """Represent a Neptune Loader Get-Status request,
    see https://docs.aws.amazon.com/neptune/latest/userguide/load-api-reference-status-requests.html"""

    def __init__(self):
        self.loadId = None
        """The ID of the load job. If you do not specify a loadId, 
        a list of load IDs is returned."""

        self.details = None
        """Include details beyond overall status.
        Allowed values: TRUE, FALSE.
        Default value: FALSE."""

        self.errors = None
        """Include the list of errors.
        Allowed values: TRUE, FALSE.
        Default value: FALSE.

        The list of errors is paged.
        The page and errorsPerPage parameters allow you to 
        page through all the errors."""

        self.page = None
        """The error page number. 
        Only valid with the errors parameter set to TRUE.
        Allowed values: Positive integers.
        Default value: 1."""

        self.errorsPerPage = None
        """The number of errors per each page. 
        Only valid with the errors parameter set to TRUE.
        Allowed values: Positive integers.
        Default value: 10."""

        self.limit = None
        """The number of load ids to list. 
        Only valid when requesting a list of load IDs 
        by sending a GET request with no loadId specified.
        Allowed values: Positive integers from 1 through 100.
        Default value: 100."""

        self.includeQueuedLoads = None
        """An optional parameter that can be used to exclude 
        the load IDs of queued load requests when a list of 
        load IDs is requested."""

    def __str__(self):
        return f"LoaderGetStatusRequest({self.loadId})"

    def json(self):
        return json.dumps(self, default=lambda o: o.__dict__, sort_keys=True, indent=4)

    def validate(self):
        pass
