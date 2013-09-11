APPNAME = 'rv6'
VERSION = '0.0.1'

# Custom tasks

from waflib import Task, TaskGen
import re

# Simple non recursive include scanner for nasm and c
def nasm_scan(task):
    deps = []
    node = task.inputs[0]

    include = re.compile('\A%include\s+"([a-zA-Z0-9_.]+)"')

    with open(node.abspath(), "r") as f:
        for line in f:
            match = include.match(line)
            if match:
                dep = node.parent.find_resource(match.group(1))
                if not dep:
                    raise "Could not find dependency resource"
                deps.append(dep)
    return (deps, [])


def c_scan(task):
    deps = []
    node = task.inputs[0]

    include = re.compile('\A#include\s+["<]([a-zA-Z0-9_.]+)[">]')

    with open(node.abspath(), "r") as f:
        for line in f:
            match = include.match(line)
            if match:
                dep = node.parent.find_resource(match.group(1))
                if not dep:
                    raise "Could not find dependency resource"
                deps.append(dep)
    return (deps, [])


def rs_scan(task):
    deps = []
    node = task.inputs[0]

    return (rs_rec_scan(node,deps), [])


def rs_rec_scan(node, deps):
    include = re.compile('\Amod\s+([a-zA-Z0-9_]+)')
    with open(node.abspath(), "r") as f:
        for line in f:
            match = include.match(line)
            if match:
                dep = node.parent.find_resource(match.group(1) + '.rs')
                if not dep:
                    raise "Could not find dependency resource"
                rec_deps = rs_rec_scan(dep, deps)
                deps.append(dep)
    return deps

@TaskGen.taskgen_method
@TaskGen.feature('bla')
def f_rust(self):
    setattr(self, 'scan', rs_scan)


@TaskGen.taskgen_method
@TaskGen.feature('nasm')
def f_nasm(self):
    setattr(self, 'scan', nasm_scan)


@TaskGen.taskgen_method
@TaskGen.feature('cobject')
def f_c(self):
    setattr(self, 'scan', c_scan)




# Allow to run scripts from the same folder via rule
@TaskGen.taskgen_method
@TaskGen.feature('local')
def local_f(self):
    setattr(self, 'rule', self.path.abspath()+'/'+getattr(self, 'rule', None))


# The build script
def configure(ctx):
    # Find other programs needed for the build process
    ctx.find_program('nasm', var='NASM')
    ctx.find_program('gcc', var='CC')
    ctx.find_program('ld', var='LD')
    ctx.find_program('rustc', var='RUSTC')
    ctx.find_program('qemu-system-i386', var='QEMU')
    ctx.find_program('rustc', var='RUSTC')
    ctx.find_program('objcopy', var='OBJCOPY')

    ctx.env.NASMFLAGS = '-f elf32'
    ctx.env.RUSTFLAGS = '-O --target i386-intel-linux --lib -c'
    ctx.env.CFLAGS = ['-fno-pic', '-static', '-fno-builtin',
                      '-fno-strict-aliasing', '-Wall', '-MD', '-ggdb',
                      '-m32', '-Werror', '-fno-omit-frame-pointer',
                      '-fno-stack-protector', '-O', '-nostdinc']

    print('Successfully configured project')


def build(ctx):
    ctx.recurse("src")


def run(ctx):
    ctx(rule='${QEMU} ${SRC}', source='src/rv6.img',  always = True)


from waflib.Build import BuildContext
class runner(BuildContext):
        cmd = 'run'
        fun = 'run'

