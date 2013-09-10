rv6
===

rv6 is a kernel & operating system written (so far it is not much more than rustboot + entrypage) in rust. It is based on the educational xv6 operating system from the MIT course 6.828: Operating System Engineering (MIT License). It is also based on zero.rs and rustboot (both MIT License) and some x86 assembler snippets from the OSDev wiki (all PD). The amount of C code is kept minimal. So far it is only used to bridge some global variables between x86 assembler and rust at the early start of the kernel as well as for the 512 bytes boot sector.

Disclaimer
----------

This is just a hobby project which I started to learn waf, rust and os development at the same time. Not sure if that is the best approach (for os development and learning these three topics) as things might turn out very ugly... ;)