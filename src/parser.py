def parse_dg(data):
    pages = paginate(data)

    for page in pages:
        print(page)


def paginate(data) -> list[list[str]]:
    pages = []
    page = []

    def push_page(page):
        # don't push page if empty
        if list(filter(lambda x: x != "\n", page)):
            pages.append(page)

    for line in data.readlines():
        if line == "---\n":
            push_page(page)
            page = []
        else:
            page.append(line)

    # push the last page if not empty
    push_page(page)

    return pages
