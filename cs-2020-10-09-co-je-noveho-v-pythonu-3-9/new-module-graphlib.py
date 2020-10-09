#!/usr/bin/env python3
#
# Python 3.9: New module: graphlib
#

import graphlib

#
#           C --> F
#           ^      \
#          / \      v
#   A --> B   D --> G
#          \ /
#           v
#           E
#
graph = {
    'A': [],
    'B': ['A'],
    'C': ['B', 'D'],
    'D': [],
    'E': ['B', 'D'],
    'F': ['C'],
    'G': ['D', 'F'],
}
ts = graphlib.TopologicalSorter(graph)
print(list(ts.static_order())) # ['A', 'D', 'B', 'C', 'E', 'F', 'G']
