#!/usr/bin/env python3

# Python <= 3.6:
#
#   ImportError: cannot import name 'foo'
#
# Python 3.7:
#
#   ImportError: cannot import name 'foo' from 'multiprocessing'
#                (/usr/lib/python3.7/multiprocessing/__init__.py)

from multiprocessing import foo
