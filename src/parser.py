def parse_dg(file):
    pages = paginate(file)


def paginate(file):
    pages = []
    page = []

    for line in file:
        if line == "---\n":
            pages.append(page)
            page = []
        else:
            page.append(line)

    return pages
