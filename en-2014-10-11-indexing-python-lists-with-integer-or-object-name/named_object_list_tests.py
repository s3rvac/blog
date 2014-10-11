"""
    named_object_list_tests
    ~~~~~~~~~~~~~~~~~~~~~~~

    Unit tests for the :mod:`named_object_list` module.

    :copyright: © 2014 by Petr Zemek <s3rvac@gmail.com>
    :license: BSD, see LICENSE for more details
"""

import unittest

from named_object_list import NamedObjectList


class File:
    """A dummy class to be used in :class:`NamedObjectList` tests."""

    def __init__(self, name):
        self.name = name

    # Makes assertion messages more useful.
    def __repr__(self):
        return self.name


class NamedObjectListTests(unittest.TestCase):
    """Tests for `_NamedObjectList`."""

    def setUp(self):
        self.file_a = File('a.txt')
        self.file_b = File('b.txt')
        self.file_c = File('c.txt')
        self.files = NamedObjectList([
            self.file_a,
            self.file_b,
            self.file_c
        ])

    # __getitem__

    def test_getitem_by_int_returns_correct_object_when_object_exists(self):
        self.assertEqual(self.files[0], self.file_a)
        self.assertEqual(self.files[1], self.file_b)
        self.assertEqual(self.files[2], self.file_c)

    def test_getitem_by_int_raises_index_error_when_no_such_object_exist(self):
        with self.assertRaises(IndexError):
            self.files[3]

    def test_getitem_by_name_returns_correct_object_when_object_exists(self):
        self.assertEqual(self.files['a.txt'], self.file_a)
        self.assertEqual(self.files['b.txt'], self.file_b)
        self.assertEqual(self.files['c.txt'], self.file_c)

    def test_getitem_by_name_raises_index_error_when_no_such_object_exist(self):
        with self.assertRaises(IndexError):
            self.files['no_such_file.txt']

    # __setitem__

    def test_setitem_by_int_returns_correct_object_when_object_exists(self):
        self.files[0] = self.file_b
        self.assertEqual(self.files[0], self.file_b)

    def test_setitem_by_int_raises_index_error_when_no_such_object_exist(self):
        with self.assertRaises(IndexError):
            self.files[3] = self.file_b

    def test_setitem_by_name_returns_correct_object_when_object_exists(self):
        self.files['a.txt'] = self.file_b
        self.assertEqual(self.files[0], self.file_b)

    def test_setitem_by_name_raises_index_error_when_no_such_object_exist(self):
        with self.assertRaises(IndexError):
            self.files['no_such_file.txt'] = self.file_b

    # __delitem__

    def test_delitem_by_int_returns_correct_object_when_object_exists(self):
        del self.files[0]
        self.assertEqual(self.files[0], self.file_b)

    def test_delitem_by_int_raises_index_error_when_no_such_object_exist(self):
        with self.assertRaises(IndexError):
            del self.files[3]

    def test_delitem_by_name_returns_correct_object_when_object_exists(self):
        del self.files['a.txt']
        self.assertEqual(self.files[0], self.file_b)

    def test_delitem_by_name_raises_index_error_when_no_such_object_exist(self):
        with self.assertRaises(IndexError):
            del self.files['no_such_file.txt']
