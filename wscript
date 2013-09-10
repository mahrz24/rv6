APPNAME = 'rv6'
VERSION = '0.0.1'

# Custom tasks

from waflib import Task, TaskGen

# Allow to run scripts from the same folder via rule
@TaskGen.taskgen_method
@TaskGen.feature('local')
def methodName(self):
    setattr(self, 'rule', self.path.abspath()+'/'+getattr(self, 'rule', None))



# The build script

def options(opt):
    opt.load('compiler_c')

def configure(ctx):
    ctx.load('compiler_c')

    # Find other programs needed for the build process
    ctx.find_program('nasm', var='NASM')
    ctx.find_program('ld', var='LD')
    ctx.find_program('rustc', var='RUSTC')
    ctx.find_program('qemu-system-i386', var='QEMU')
    ctx.find_program('rustc', var='RUSTC')
    ctx.find_program('objcopy', var='OBJCOPY')

    ctx.env.NASM_FLAGS = '-f elf32'
    ctx.env.RUST_FLAGS = '-O --target i386-intel-linux --lib -c'

    print('Successfully configured project')

def build(ctx):
    ctx.recurse("src")

def run(ctx):
    ctx(rule='${QEMU} ${SRC}', source='src/rv6.img',  always = True)

from waflib.Build import BuildContext
class runner(BuildContext):
        cmd = 'run'
        fun = 'run'

