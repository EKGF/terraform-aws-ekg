def convert_neptune_error_code(error_json):
    """Convert the recoverable/retryable Neptune errors to a code."""
    # Response will be something like:
    # {
    #     "detailedMessage": "Failed to start new load for the source s3:///...somefile.ttl.
    #                         Max load task queue size limit breached. Limit is 64",
    #     "code": "BadRequestException",
    #     "requestId": "d0c61d09-6fed-01bd-feb4-a6fd25d1a72f",
    #     "message": "Failed to start new load for the source s3://..somefile.ttl.
    #                 Max load task queue size limit breached. Limit is 64"
    # }
    message = error_json['message']
    if "Max load task queue size limit breached" in message:
        return "MaxLoadTaskQueueSizeLimitBreached"
    if "Max concurrent load limit breached" in message:
        return "MaxConcurrentLoadLimitBreached"
    return "Unknown"
