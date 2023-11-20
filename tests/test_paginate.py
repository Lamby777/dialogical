from io import StringIO
import unittest

from .context import add_path

add_path()

from dialogical import parser

ONE_PAGE = StringIO(
    """---
%Interaction

---
NAME Deez
VOX Deez

When the words are sus
"""
)

MANY_PAGES = StringIO(
    """---
%Interaction

---
NAME Deez
VOX Deez

When the words are sus

---
NAME Gamer

Words go brrr

---

When the imposter is sus

---
###

// Another Page
Echo hello world

###
---
NAME Siva
VOX Siva

---

"""
)


class PaginateTest(unittest.TestCase):
    def test_one_page(self):
        res = parser.paginate(ONE_PAGE)
        self.assertEqual(
            res,
            [
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
            ],
        )

    def test_many_pages(self):
        res = parser.paginate(MANY_PAGES)
        self.assertEqual(
            res,
            [
                [
                    "%Interaction\n",
                    "\n",
                ],
                [
                    "NAME Deez\n",
                    "VOX Deez\n",
                    "\n",
                    "When the words are sus\n",
                    "\n",
                ],
                [
                    "NAME Gamer\n",
                    "\n",
                    "Words go brrr\n",
                    "\n",
                ],
                [
                    "\n",
                    "When the imposter is sus\n",
                    "\n",
                ],
                [
                    "###\n",
                    "\n",
                    "// Another Page\n",
                    "Echo hello world\n",
                    "\n",
                    "###\n",
                ],
                [
                    "NAME Siva\n",
                    "VOX Siva\n",
                    "\n",
                ],
            ],
        )


if __name__ == "__main__":
    unittest.main()
