import json

from ..general.input_error import InputError


class LoadRequestError(InputError):
    """Exception raised for errors in the load request."""

    def __init__(self, var_name: str):
        self.status_text = f"Load request error: {var_name} not set"
        super().__init__(500, self.status_text)


class LoadRequest:
    """Represent an RDF load request"""

    def __init__(self):
        self.invoked_function_arn = None
        self.neptune_s3_iam_role_arn = None
        self.rdf_load_sfn_arn = None
        self.s3_uri = None
        self.rdf_format = None
        self.s3_bucket_region = None
        self.ekg_id_base_internal = None

    def __str__(self):
        return f"LoadRequest({self.neptune_load_request()})"

    def json(self):
        return json.dumps(self, default=lambda o: o.__dict__, sort_keys=True, indent=4)

    def validate(self):
        if not self.invoked_function_arn:
            raise LoadRequestError("invoked_function_arn")
        if not self.neptune_s3_iam_role_arn:
            raise LoadRequestError("neptune_s3_iam_role_arn")
        if not self.rdf_load_sfn_arn:
            raise LoadRequestError("rdf_load_sfn_arn")
        if not self.s3_uri:
            raise LoadRequestError("s3_uri")
        if not self.rdf_format:
            raise LoadRequestError("rdf_format")
        if not self.s3_bucket_region:
            raise LoadRequestError("s3_bucket_region")
        if not self.ekg_id_base_internal:
            raise LoadRequestError("ekg_id_base_internal")

    def neptune_load_request(self):
        """
        Return a Neptune load request in JSON format.

        See https://docs.aws.amazon.com/neptune/latest/userguide/load-api-reference-load.html

        :return: Neptune load request in JSON format
        """
        self.validate()
        return json.dumps({
            "source": self.s3_uri,
            "format": self.rdf_format,
            "iamRoleArn": self.neptune_s3_iam_role_arn,
            "mode": "NEW",
            "region": self.s3_bucket_region,
            "failOnError": "TRUE",
            "parallelism": "HIGH",
            "parserConfiguration": {
                "baseUri": f"{self.ekg_id_base_internal}/",
                # Just load the whole file into its own named graph for provenance reasons,
                # triggered processes will transform and move it to more appropriate named graphs
                "namedGraphUri": self.s3_uri
            },
            "updateSingleCardinalityProperties": "FALSE",
            "queueRequest": "TRUE",
            "dependencies": []
        }, indent=4)
