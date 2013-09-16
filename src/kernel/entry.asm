; Sources from http://wiki.osdev.org/Bare_Bones_with_NASM & xv6's entry.S
; Declare constants used for creating a multiboot header.
%define MBALIGN   1<<0                   ; align loaded modules on page boundaries
%define MEMINFO   1<<1                   ; provide memory map
%define FLAGS     MBALIGN | MEMINFO      ; this is the Multiboot 'flag' field
%define MAGIC     0x1BADB002             ; 'magic number' lets bootloader find the header
%define CHECKSUM -(MAGIC + FLAGS)        ; checksum of above, to prove we are multiboot

%include "memory_c.asm"
%include "params_c.asm"

extern entrypgdir
extern main

global _GLOBAL_OFFSET_TABLE_
global __morestack
global abort
global memcmp
global memcpy
global malloc
global free
global start

; Declare a header as in the Multiboot Standard
section .multiboot
align 4
  dd MAGIC
  dd FLAGS
  dd CHECKSUM

; The linker script specifies _start as the entry point to the kernel and the
; bootloader will jump to this position once the kernel has been loaded. It
; doesn't make sense to return from this function as the bootloader is gone.
section .text
global _start

_start equ (kentry-KERNBASE)

global kentry
kentry:

; Clear the screen with stars
  mov edi, 0xb8000
  mov ecx, 80*25*2
  mov al, 0x0F
  rep stosb

; Turn on page size extension for 4Mbyte pages
  mov    eax, cr4
  or     eax, CR4_PSE
  mov    cr4, eax

; Set page directory
  mov    eax, (entrypgdir-KERNBASE)
  mov    cr3, eax

; Turn on paging.
  mov    eax, cr0
  or     eax, (CR0_PG | CR0_WP)
  mov    cr0, eax
; Set up the stack pointer.
  mov    esp, stack + KSTACKSIZE

; Jump to main(), and switch to executing at
; high addresses. The indirect call is needed because
; the assembler produces a PC-relative instruction
; for a direct jump.

  mov    [gs:0x30], dword 0
  mov    eax, main
  jmp    eax

_GLOBAL_OFFSET_TABLE_:

__morestack:

abort:
    jmp $

memcmp:
    jmp $

memcpy:
    jmp $

malloc:
    jmp $

free:
    jmp $


common stack KSTACKSIZE