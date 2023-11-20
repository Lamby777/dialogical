def parse_dg(data):
    pages = paginate(data)

    for page in pages:
        print(page)


def paginate(data) -> list[list[str]]:
    pages = []
    page = []

    for line in data.readlines():
        if line == "---\n":
            # don't push page if empty
            if filter(lambda x: x != "\n", page):
                pages.append(page)
                page = []
        else:
            page.append(line)

    # push the last page if not empty
    pages.append(page)

    return pages
