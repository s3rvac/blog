#!/usr/bin/env python3
#
# An HTTP client that checks that the content of the response has at least
# Content-Length bytes.
#
# For requests 2.x (not needed for requests 3.x).
#

import requests
import sys

response = requests.get('http://localhost:8080/')
if not response.ok:
    sys.exit('error: HTTP {}'.format(response.status_code))

# Check that we have read all the data as the requests library does not
# currently enforce this.
expected_length = response.headers.get('Content-Length')
if expected_length is not None:
    # We cannot use len(response.content) as this would not work when the body
    # of the response was compressed (e.g. when the response has
    # `Content-Encoding: gzip`).
    actual_length = response.raw.tell()
    expected_length = int(expected_length)
    if actual_length < expected_length:
        raise IOError(
            'incomplete read ({} bytes read, {} more expected)'.format(
                actual_length,
                expected_length - actual_length
            )
        )

print(response.headers)
print(response.content)
print(len(response.content))
