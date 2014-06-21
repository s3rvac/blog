"""
    chdir
    ~~~~~

    Contains a context manager that performs actions in the given directory.

    :copyright: © 2014 by Petr Zemek <s3rvac@gmail.com>
    :license: BSD, see LICENSE for more details
"""

import contextlib
import os


@contextlib.contextmanager
def chdir(dir):
    """A context manager that performs actions in the given directory.

    Example:

    >>> import os
    >>> print(os.getcwd())
    /
    >>> with chdir('/tmp'):
    ...     print(os.getcwd())
    ...
    /tmp
    >>> print(os.getcwd())
    /
    """
    orig_cwd = os.getcwd()
    os.chdir(dir)
    try:
        yield
    finally:
        os.chdir(orig_cwd)
