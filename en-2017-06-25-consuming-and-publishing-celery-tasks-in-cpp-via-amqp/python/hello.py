#
# Sends a request to call hello() from within a worker.
#

import sys

from tasks import hello

if __name__ == '__main__':
    if len(sys.argv) != 3:
        print('usage: {} NAME AGE'.format(sys.argv[0]))
        sys.exit(1)

    # By calling hello.delay(), we request hello() to be executed in a worker
    # rather than executing it directly in the current process, which is what a
    # direct call to hello() would do.
    # http://docs.celeryproject.org/en/latest/userguide/calling.html
    hello.delay(sys.argv[1], int(sys.argv[2]))
