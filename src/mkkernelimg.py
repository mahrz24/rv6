#!/usr/bin/python

import sys
import os

out_file = sys.argv[3]
bs_file = sys.argv[1]
kernel_file = sys.argv[2]
os.system('dd if=/dev/zero of=' + out_file + ' count=10000')
os.system('dd if=' + bs_file + ' of=' + out_file + ' conv=notrunc ')
os.system('dd if=' + kernel_file + ' of=' + out_file + ' seek=1 conv=notrunc ')
