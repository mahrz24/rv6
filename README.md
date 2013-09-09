rv6
===

rv6 is a kernel & operating system written (so far it is not much more than rustboot + entrypage) in rust. It is based on the educational xv6 operating system from the MIT course 6.828: Operating System Engineering (MIT License). It is also based on zero.rs and rustboot (both MIT License) and some x86 assembler snippets from the OSDev wiki (all PD). The amount of C code is kept minimal and so far is only used to bridge some global variables between x86 assembler and rust as well as for the 512 bytes boot sector.