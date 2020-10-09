#!/usr/bin/env python3
#
# Python 3.9: Dictionary merge & update operators
#

d1 = {'a': 1, 'b': 1}
d2 = {'b': 2, 'c': 2}

# Instead of
d = d1.copy()
d.update(d2)
# which requires a temporary variable, or
d = {**d1, **d2}
# which is ugly and not easily discoverable, one can now write
d = d1 | d2
print(d)

# If you want to update a dict in place, you can write
d1 |= d2
print(d1)
