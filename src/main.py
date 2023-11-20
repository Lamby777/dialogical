#!/usr/bin/env python3

########################################
#            | dialogical |            #
# Dialogue toolkit for game developers #
#                                      #
#             - &Cherry, 11/19/2023 <3 #
########################################

VERSION = "0.0.3"


from argparse import ArgumentParser
import sys

from parser import parse_dg


def main():
    args = parse_args()

    if args.version:
        print(f"dialogical v{VERSION}")
        sys.exit(0)

    # not printing version... so we're parsing a file
    with sys.stdin if args.stdin else open(args.file) as target:
        print(f"parsing {target.name}...")
        parse_dg(target)

    # output the results
    with sys.stdout if not args.output else open(args.output, "w") as output:
        print(f"writing to {output.name}...")
        output.write("hi")


def parse_args():
    parser = ArgumentParser()
    parser.add_argument("-o", "--output", help="output file name")

    g = parser.add_mutually_exclusive_group(required=True)
    g.add_argument("-v", "--version", help="version info", action="store_true")
    g.add_argument("-s", "--stdin", help="read from stdin", action="store_true")
    g.add_argument("file", help="definition file", nargs="?")

    # show help if no args given
    # "borrowed, not stolen"
    # - Ferris the Crab
    if len(sys.argv) == 1:
        parser.print_help(sys.stderr)
        sys.exit(1)

    return parser.parse_args()


if __name__ == "__main__":
    main()
