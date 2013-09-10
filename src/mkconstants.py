#!/usr/bin/python

import argparse
import re

parser = argparse.ArgumentParser(
    description='Convert a .consts file to rust, nasm or c header file.')
parser.add_argument('infile', type=str,
                    help='the consts file')
parser.add_argument('outfile', type=str, nargs='+',
                    help='the rust, nasm or c header file names')

args = parser.parse_args()

cre = re.compile("\A(\w+)\s+(\S+)\s.*")
constants = []

with open(args.infile, "r") as fi:
    for line in fi:
        match = cre.match(line)
        if match:
            constants.append([match.group(1), match.group(2)])

for outfile in args.outfile:
    with open(outfile, "w") as fo:
        out_type = outfile.split('.')[1]
        for const in constants:
            if out_type == "h":
                fo.write('#define %s %s\n' %(const[0], const[1]))
            elif out_type == "asm":
                fo.write('%%define %s %s\n' %(const[0], const[1]))
            elif out_type == "rs":
                fo.write('pub static %s: uint = %s;\n' %(const[0], const[1]))