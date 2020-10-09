#!/usr/bin/env python3
#
# Python 3.9: Type hinting generics in standard collections
#

import typing

# Instead of using e.g. typing.List:
def concatenate(strings: typing.List[str]) -> str:
    ...
# one can now use directly built-in types:
def concatenate(strings: list[str]) -> str:
    ...
