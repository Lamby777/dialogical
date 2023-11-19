#!/usr/bin/env python3

########################################
#            | dialogical |            #
# Dialogue toolkit for game developers #
#                                      #
#             - &Cherry, 11/19/2023 <3 #
########################################

VERSION = "0.0.1"


from argparse import ArgumentParser
import sys


def main():
    args = parse_args()

    if args.v:
        print(f"dialogical v{VERSION}")
        sys.exit(0)

    # not printing version... so we're parsing a file
    with sys.stdin if args.stdin else open(args.file) as target:
        print(f"parsing {target.name}...")
        # parse(target)

    # output the results
    with sys.stdout if not args.output else open(args.output, "w") as output:
        print(f"writing to {output.name}...")
        output.write("hi")


def parse_args():
    parser = ArgumentParser()
    parser.add_argument("-o", "--output", help="output file name")

    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("-v", help="version info", action="store_true")
    group.add_argument("-s", "--stdin", help="read from stdin", action="store_true")
    group.add_argument("file", help="definition file", nargs="?")

    return parser.parse_args()


if __name__ == "__main__":
    main()
