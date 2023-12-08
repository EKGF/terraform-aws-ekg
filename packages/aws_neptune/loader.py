def convert_successful_loader_output(loader_output):
    """Convert neptune
     load output to the same format we're using for the errors, more or less"""
    return {
        "statusCode": 200,
        "statusDetail": loader_output['payload']
    }
