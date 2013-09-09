; Segments

%define SEG_NULLASM dq 0

%macro SEG_ASM 3 ; Type, Base, Lim
        dw (((%3) >> 12) & 0xffff)
        dw ((%2) & 0xffff)
        db (((%2) >> 16) & 0xff)
        db (0x90 | (%1))
        db (0xC0 | (((%3) >> 28) & 0xf))
        db (((%2) >> 24) & 0xff)
%endmacro

%define STA_X     0x8  ; Executable segment
%define STA_E     0x4  ; Expand down (non-executable segments)
%define STA_C     0x4  ; Conforming code segment (executable only)
%define STA_W     0x2  ; Writeable (non-executable segments)
%define STA_R     0x2  ; Readable (executable segments)
%define STA_A     0x1  ; Accessed

; Control Register Flags

%define CR0_PE    0x00000001 ; Protection enable

%define SEG_KCODE 1  ; kernel Code
%define SEG_KDATA 2  ; kernel Data+Stack

use16 ; 16-bit mode


global start

extern bootmain


start:
  ; Disable interrupts
  cli

  ; Zero segments registers
  xor    ax, ax
  mov    ds, ax ; data
  mov    es, ax ; extra
  mov    ss, ax ; stack

; Enable the A20 line
seta201:
  in     al, 0x64
  test   al, 2
  jnz    seta201

  mov    al, 0xd1
  out    0x64, al

seta202:
  in     al, 0x64
  test   al, 2
  jnz    seta202

  mov    al, 0xd1
  out    0x64, al



; Switch from real to protected mode
  lgdt   [gdtdesc]
  mov    eax, cr0
  or     eax, CR0_PE
  mov    cr0, eax


  jmp    (SEG_KCODE<<3):start32

start32:
use32
  mov    eax, SEG_KDATA<<3
  mov    ds, eax
  mov    es, eax
  mov    ss, eax
  mov    ax, 0
  mov    fs, eax
  mov    gs, eax

; Clear the screen
  mov edi, 0xb8000
  mov ecx, 80*25*2
  mov al, 0
  rep stosb

  mov    esp, start
  call   bootmain

; Should not happen, trigger breakpoint
  mov    ax, 0x8a00
  mov    dx, ax
  out    dx, ax
  mov    ax, 0x8a00
  out    dx, ax

gdt:
    ; null entry
    SEG_NULLASM
    ; code entry
    SEG_ASM STA_X | STA_R, 0x0, 0xffffffff
    SEG_ASM STA_W,         0x0, 0xffffffff

gdtdesc:
    dw (gdtdesc - gdt - 1) ; Size
    dd gdt ; Offset