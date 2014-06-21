"""
    chdir2
    ~~~~~~

    An alternative implementation of :func:`chdir.chdir`.

    :copyright: © 2014 by Petr Zemek <s3rvac@gmail.com>
    :license: BSD, see LICENSE for more details
"""

import os


class chdir2():
    """An alternative implementation of :func:`chdir.chdir`."""

    def __init__(self, dir):
        self.dir = dir

    def __enter__(self):
        self.orig_cwd = os.getcwd()
        os.chdir(self.dir)

    def __exit__(self, *exc_info):
        os.chdir(self.orig_cwd)
