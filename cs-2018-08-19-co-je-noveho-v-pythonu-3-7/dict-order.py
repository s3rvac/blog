#!/usr/bin/env python3

d = {'a': 1, 'b': 2, 'c': 3}

# Unspecified order in Python <= 3.6, guaranteed order ['a', 'b', 'c'] in Python 3.7.
print([k for k in d])
