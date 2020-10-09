#!/usr/bin/env python3
#
# Python 3.9: Relaxing grammar restrictions on decorators
#

decorators = [
    # Does nothing, just for illustration.
    lambda func: func
]

# Python <= 3.8 raises SyntaxError when the decorator expression contains
# something other than dotted identifiers (optionally followed by a function
# call). Since Python 3.9, you can write e.g. this:
@decorators[0]
def foo():
    ...
