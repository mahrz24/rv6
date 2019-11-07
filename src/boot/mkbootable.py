#!/usr/bin/python

import sys

in_filename = sys.argv[1]
out_filename = sys.argv[2]

chunksize = 1000

with open(in_filename, "rb") as fi:
    with open(out_filename, "wb") as fo:
        chunk = bytearray(fi.read(chunksize))
        chunksize = len(chunk)

        if chunksize > 510:
            print('Error: Bootblock too large (must be <= 510 bytes).')
            sys.exit(1)

        if chunksize < 510:
            for i in range(chunksize, 510):
                chunk.append(0)

        chunk.append(0x55)
        chunk.append(0xAA)
        fo.write(chunk)
