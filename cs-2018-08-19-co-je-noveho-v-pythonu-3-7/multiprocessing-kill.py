#!/usr/bin/env python3

import multiprocessing

def foo():
    while True:
        pass

p = multiprocessing.Process(target=foo)
p.start()
p.kill()
