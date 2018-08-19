#!/usr/bin/env python3

import dataclasses

@dataclasses.dataclass
class Point:
    x: int = 0
    y: int = 0

p1 = Point(2, 1)
p2 = Point(2, 3)

# Attribute access:
print(p1.y)  # 1
p1.y = 3
print(p1.y)  # 3

# Representation:
print(p1)    # Point(x=2, y=3)

# Comparison:
print(p1 == p2)  # True
print(p1 != p2)  # False
