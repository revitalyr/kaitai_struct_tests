# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from bits_simple_le import _schema

class TestBitsSimpleLe(unittest.TestCase):
    def test_bits_simple_le(self):
        r = _schema.parse_file('src/fixed_struct.bin')

        self.assertEqual(r.byte_1, 80)
        self.assertEqual(r.byte_2, 65)
        self.assertEqual(r.bits_a, True)
        self.assertEqual(r.bits_b, 1)
        self.assertEqual(r.bits_c, 4)
        self.assertEqual(r.large_bits_1, 331)
        self.assertEqual(r.spacer, 3)
        self.assertEqual(r.large_bits_2, 393)
        self.assertEqual(r.normal_s2, -1)
        self.assertEqual(r.byte_8_9_10, 4407632)
        self.assertEqual(r.byte_11_to_14, 760556875)
        self.assertEqual(r.byte_15_to_19, 1099499455812)
        self.assertEqual(r.byte_20_to_27, 18446744073709551615)
        self.assertEqual(r.test_if_b1, 123)