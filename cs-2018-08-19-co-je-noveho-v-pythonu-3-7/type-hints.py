#!/usr/bin/env python3

from __future__ import annotations

# OK in Python 3.7, `NameError: name 'MyClass' is not defined` in Python <= 3.6.

def foo() -> MyClass:
    return MyClass()

class MyClass:
    pass
