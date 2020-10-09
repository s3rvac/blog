#!/usr/bin/env python3
#
# Python 3.9: Flexible function and variable annotations
#

import typing

# Based on
# https://www.python.org/dev/peps/pep-0593/#combining-runtime-and-static-uses-of-annotations
#
# Tools can then serialize variables annotated with `UnsignedShort` as a 16-bit
# unsigned integer.
#
UnsignedShort = typing.Annotated[int, ('u', 16)]
