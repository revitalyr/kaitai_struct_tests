# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from default_endian_mod import _schema

class TestDefaultEndianMod(unittest.TestCase):
    def test_default_endian_mod(self):
        r = _schema.parse_file('src/fixed_struct.bin')
        self.assertEqual(r.main.one, 1262698832)
        self.assertEqual(r.main.nest.two, -52947)
        self.assertEqual(r.main.nest_be.two, 1346454347)