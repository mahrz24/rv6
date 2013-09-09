APPNAME = 'rv6'
VERSION = '0.0.1'

def configure(ctx):
  ctx.find_program('nasm', var='NASM')
  ctx.find_program('ld', var='LD')
  ctx.find_program('rustc', var='RUSTC')
  ctx.find_program('gcc', var='CC')
  ctx.find_program('qemu-system-i386', var='QEMU')
  ctx.find_program('objcopy', var='OBJCOPY')
  print('Successfully configured project')

def build(ctx):
  ctx.recurse("src")

def run(ctx):
  ctx(rule='${QEMU} ${SRC}', source='src/rv6.img',  always = True)


from waflib.Build import BuildContext
class runner(BuildContext):
        cmd = 'run'
        fun = 'run'