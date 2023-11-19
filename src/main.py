#!/usr/bin/env python3

########################################
#            - dialogical -            #
#                                      #
# Dialogue toolkit for game developers #
#                                      #
#             - &Cherry, 11/19/2023 <3 #
#                                      #
########################################


from argparse import ArgumentParser


def parse_args():
    parser = ArgumentParser()
    parser.add_argument("file", help="definition file")
    parser.add_argument("-o", "--output", help="output file name")
    parser.add_argument("-v", "--version", help="version info", action="store_true")
    return parser.parse_args()


def main():
    args = parse_args()
    print(args)


if __name__ == "__main__":
    main()
