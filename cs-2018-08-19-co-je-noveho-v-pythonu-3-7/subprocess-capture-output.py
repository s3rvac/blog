#!/usr/bin/env python3

import subprocess

p = subprocess.run(['echo', 'hello'], capture_output=True)
print(p.stdout)  # b'hello\n'
