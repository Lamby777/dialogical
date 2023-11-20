import unittest

# This is the class we want to test. So, we need to import it
from src.parser import paginate

ONE_PAGE = """---
%Interaction

---
NAME Deez
VOX Deez

When the words are sus
"""


class PaginateTest(unittest.TestCase):
    def test_one_page(self):
        res = paginate(ONE_PAGE)
        self.assertEqual(
            res,
            [
                "%Interaction\n",
                "\n",
            ],
            [
                "NAME Deez\n",
                "VOX Deez\n",
                "\n",
                "When the words are sus\n",
            ],
        )


if __name__ == "__main__":
    unittest.main()
