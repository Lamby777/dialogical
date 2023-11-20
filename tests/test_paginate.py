import unittest

# This is the class we want to test. So, we need to import it
from src.parser import parse_dg


class Test(unittest.TestCase):
    def test_deez_nuggets(self):
        self.assertEqual(1, 1)


if __name__ == "__main__":
    unittest.main()
