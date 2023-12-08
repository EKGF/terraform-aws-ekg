class InputError(Exception):
    """Exception raised for errors in the input.

    Attributes:
        status -- status code
        message -- explanation of the error
    """

    def __init__(self, status: int, status_text: str):
        self.status = status
        self.status_text = status_text
        super().__init__(self.status_text)

    def json(self):
        return {
            "statusCode": self.status,
            "statusText": self.status_text
        }
