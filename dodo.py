import glob
import os

DOIT_CONFIG = {'default_tasks': ['build']}

QEMU = 'qemu-system-i386'
LD = 'ld'
CC = 'gcc'
OBJCOPY = 'objcopy'
NASM = 'nasm'
RUSTC = 'rustc'

NASMFLAGS = '-f elf32'
CFLAGS = ' '.join(['-fno-pic -static -fno-builtin ',
         '-fno-strict-aliasing -Wall -MD -ggdb ',
         '-m32 -Werror -fno-omit-frame-pointer ',
         '-fno-stack-protector -O -nostdinc '])
RUSTFLAGS = '-O --target i386-intel-linux --lib -c -Z debug-info'

def task_provide_memory_constants():
    return {'actions': ['src/mkconstants.py '
                        'src/constants/memory.consts '
                        'src/boot/memory_c.asm '
                        'src/kernel/memory_c.asm '
                        'src/kernel/memory/memory_c.rs '
                        'src/kernel/memory_c.h'],
            'targets': ['src/boot/memory_c.asm',
                        'src/kernel/memory_c.asm',
                        'src/kernel/memory/memory_c.rs',
                        'src/kernel/memory_c.h'],
            'file_dep': ['src/constants/memory.consts'],
            'clean': True}


def task_provide_params_constants():
    return {'actions': ['src/mkconstants.py '
                        'src/constants/params.consts '
                        'src/kernel/params_c.asm '
                        'src/kernel/params_c.rs '],
            'targets': ['src/kernel/params_c.asm',
                        'src/kernel/params_c.rs'],
            'file_dep': ['src/constants/params.consts'],
            'clean': True}


def task_provide_proc_constants():
    return {'actions': ['src/mkconstants.py '
                        'src/constants/proc.consts '
                        'src/kernel/proc/proc_c.rs'],
            'targets': ['src/kernel/proc/proc_c.rs'],
            'file_dep': ['src/constants/proc.consts'],
            'clean': True}

def task_provide_constants():
    return {'actions' : [],
            'task_dep': ['provide_memory_constants',
                         'provide_params_constants',
                         'provide_proc_constants'],
            'clean': True}

def task_compile_entry():
    return {'actions': ['%s %s -Isrc/kernel/ ' % (NASM, NASMFLAGS) +
                        ' -o src/kernel/entry.o '
                        ' src/kernel/entry.asm'],
        'file_dep': ['src/kernel/entry.asm',
                     'src/kernel/memory_c.asm',
                     'src/kernel/params_c.asm'],
        'targets': ['src/kernel/entry.o'],
        'clean': True}

def task_compile_main():
    deps = glob.glob('./src/*/*.rs') + glob.glob('./src/*/*/*.rs')
    return {'actions': ['%s %s ' % (RUSTC, RUSTFLAGS) +
                        ' -o src/kernel/main.o '
                        ' src/kernel/main.rs'],
        'file_dep': deps + ['src/kernel/memory/memory_c.rs',
                            'src/kernel/proc/proc_c.rs',
                            'src/kernel/params_c.rs'],
        'targets': ['src/kernel/main.o'],
        'clean': True}

def task_compile_c():
    for f in ['cbase', 'x86']:
        yield {'name':  f,
               'actions': ['%s %s -Isrc/kernel/ ' % (CC, CFLAGS) +
                        ' -o src/kernel/%s.o ' % f +
                        ' -c src/kernel/%s.c' % f],
                'file_dep': ['src/kernel/%s.c' % f],
                'task_dep' : ['provide_constants'],
                'targets': ['src/kernel/%s.o' % f,
                            'src/kernel/%s.d' % f],
                'clean': True}

def task_link_kernel():
    return {'actions': ['%s -N -m elf_i386 -o src/kernel/kernel ' % LD +
                        ' -T src/kernel/kernel.ld '
                        'src/kernel/entry.o '
                        'src/kernel/cbase.o '
                        'src/kernel/main.o '
                        'src/kernel/x86.o'],
        'file_dep': ['src/boot/boot.ld',
                     'src/kernel/entry.o',
                     'src/kernel/cbase.o',
                     'src/kernel/main.o',
                     'src/kernel/x86.o'],
        'targets': ['src/kernel/kernel'],
        'clean': True}


def task_compile_bootmain():
    return {'actions': ['%s %s ' % (CC, CFLAGS) +
                        ' -o src/boot/bootmain.o '
                        ' -c src/boot/bootmain.c'],
        'file_dep': ['src/boot/bootmain.c'],
        'targets': ['src/boot/bootmain.o',
                    'src/boot/bootmain.d'],
        'clean': True}

def task_compile_boot():
    return {'actions': ['%s %s -Isrc/boot/ ' % (NASM, NASMFLAGS) +
                        ' -o src/boot/boot.o '
                        ' src/boot/boot.asm'],
        'file_dep': ['src/boot/boot.asm',
                     'src/boot/memory_c.asm'],
        'targets': ['src/boot/boot.o'],
        'clean': True}

def task_link_boot():
    return {'actions': ['%s -N -m elf_i386 -o src/boot/bootblock.o ' % LD +
                        ' -T src/boot/boot.ld '
                        'src/boot/boot.o '
                        'src/boot/bootmain.o'],
        'file_dep': ['src/boot/boot.ld',
                     'src/boot/boot.o',
                     'src/boot/bootmain.o'],
        'targets': ['src/boot/bootblock.o'],
        'clean': True}

def task_extract_boot():
    return {'actions': ['%s  -S -O binary -j .text ' % OBJCOPY +
                        'src/boot/bootblock.o '
                        'src/boot/bootblock.bin'],
        'file_dep': ['src/boot/bootblock.o'],
        'targets': ['src/boot/bootblock.bin'],
        'clean': True}

def task_build_boot():
    return {'actions': ['src/boot/mkbootable.py '
                        'src/boot/bootblock.bin '
                        'src/boot/bootblock'],
        'file_dep': ['src/boot/bootblock.bin'],
        'targets': ['src/boot/bootblock'],
        'clean': True}


def taks_build_kernel():
    pass


def task_build():
    return {'actions': ['src/mkkernelimg.py '
                        'src/boot/bootblock '
                        'src/kernel/kernel src/rv6.img'],
            'file_dep': ['src/boot/bootblock', 'src/kernel/kernel'],
            'targets' : ['src/rv6.img'],
            'clean': True}


def task_start():
    def qemu(smp,mem,debug):
        extra = ''
        if debug:
            extra = '-monitor stdio -gdb tcp::9573'
        os.system('%s -smp %s -m %s %s src/rv6.img' % (QEMU, smp, mem, extra))
    return {'actions': [(qemu,)],
            'task_dep': ['build'],
            'params':[{'name':'smp',
                       'short':'p',
                       'long': 'smp',
                       'default': '2'},
                      {'name':'mem',
                       'short':'m',
                       'long': 'mem',
                       'default': '512'},
                      {'name':'debug',
                       'short' : 'd',
                       'long':'debug',
                       'type': bool,
                       'default':False}]}

def task_kdbg():
    return {'actions': ['']}
