#!/usr/bin/env python3

import collections

Point = collections.namedtuple('Point', ('x', 'y'), defaults=(0, 0))

p1 = Point(1, 2)
p2 = Point()  # As if calling Point(0, 0).
