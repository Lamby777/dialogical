def parse_dg(data):
    pages = paginate(data)


def paginate(data) -> list[list[str]]:
    pages = []
    page = []

    for line in data:
        if line == "---\n":
            # don't push page if empty
            if page:
                pages.append(page)
                page = []
        else:
            page.append(line)

    return pages
