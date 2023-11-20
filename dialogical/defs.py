from . import comptime


class Page:
    """
    A single page of dialogue.

    Class is used internally for processing...
    The final output will just be a string for each page.
    """

    lines: list[str]

    def __init__(self, lines: list[str]):
        self.lines = lines


class Segment:
    """
    Multiple pages of dialogue, optionally followed by choices.
    """

    pages: list[Page]


class ComptimeScript:
    content: str

    def __init__(self, content: str):
        self.content = content

    def run(self):
        comptime.comptime_run(self.content)
