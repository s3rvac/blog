#!/usr/bin/env python3
#
# An HTTP client that does not check that the content of the response has at
# least Content-Length bytes.
#

import requests
import sys

response = requests.get('http://localhost:8080/')
if not response.ok:
    sys.exit('error: HTTP {}'.format(response.status_code))

print(response.headers)
print(response.content)
print(len(response.content))
