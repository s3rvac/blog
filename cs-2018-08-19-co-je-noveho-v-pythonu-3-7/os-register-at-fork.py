#!/usr/bin/env python3

import os

os.register_at_fork(
    before=lambda: print('before'),
    after_in_parent=lambda: print('after - parent'),
    after_in_child=lambda: print('after - child'),
)

# Prints:
#    before
#    after - parent
#    after - child

os.fork()
