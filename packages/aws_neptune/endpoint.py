import logging
import socket
from contextlib import closing
from urllib.parse import urlparse

from . import loader_check_endpoint_timeout
from ..general.input_error import InputError


def check_loader_endpoint(url):
    """Check the loader endpoint's host and port."""
    parsed_url = urlparse(url)
    host_name = parsed_url.hostname
    port = parsed_url.port

    if host_name is None:
        raise InputError(500, f"Loader endpoint's host name not specified in url [{url}]")

    logging.info(f"Checking loader endpoint's host and port: {host_name}:{str(port)}")

    try:
        machine_ip = socket.gethostbyname(host_name)
    except socket.gaierror:
        raise InputError(500, f"Loader endpoint's host name [{host_name}] could not be resolved")

    logging.info(f"Loader endpoint IP Address: {machine_ip}")

    with closing(socket.socket(socket.AF_INET, socket.SOCK_STREAM)) as sock:
        logging.info(
            f"Connecting to loader endpoint {host_name}:{str(port)} (timeout is {loader_check_endpoint_timeout} seconds)")
        sock.settimeout(loader_check_endpoint_timeout)
        if sock.connect_ex((machine_ip, port)) == 0:
            logging.info(f"Loader endpoint {host_name}:{str(port)} is available")
            return None
        else:
            logging.error(f"Loader endpoint {host_name}:{str(port)} is not available")
            return {
                "statusCode": 500,
                "statusError": f"Loader endpoint {host_name}:{str(port)} is not open"
            }
