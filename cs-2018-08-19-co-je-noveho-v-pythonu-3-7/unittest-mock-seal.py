#!/usr/bin/env python3

from unittest import mock

m = mock.Mock()
m.a = 1
mock.seal(m)
m.a = 2  # OK
m.b = 2  # AttributeError: Cannot set mock.b
