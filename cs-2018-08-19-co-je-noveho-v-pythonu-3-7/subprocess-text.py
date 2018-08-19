#!/usr/bin/env python3

import subprocess

p = subprocess.run(['echo', 'Jalapeño'], capture_output=True)
print(p.stdout)  # b'b'Jalape\xc3\xb1o\n'

p = subprocess.run(['echo', 'Jalapeño'], capture_output=True, text=True)
print(p.stdout)  # 'Jalapeño\n'
