#!/usr/bin/env python3
#
# Python version of the code that we are implementing in Rust.
#

import multiprocessing.pool

with multiprocessing.pool.ThreadPool() as pool:
    for result in pool.imap(lambda n: n * 2, [1, 2, 3, 4, 5]):
        print(result)
