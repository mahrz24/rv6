ENTRY(start)
OUTPUT_FORMAT("elf32-i386", "elf32-i386", "elf32-i386")

SECTIONS {
    . = 0x7c00;

    .text : {
        *(.text)
    }
}