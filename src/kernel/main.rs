#[link(name = "rv6kernel", vers = "0.1", author = "malte.harder")];

#[allow(ctypes)];
#[no_std];
#[no_core];

use console::vgaterm::*;

mod zero;
mod console;
mod memory;

struct Global {
  terminal: VGATerminal
}

impl Global {
  fn new() -> Global {
    Global { terminal: VGATerminal::new() }
  }
}


#[no_mangle]
pub unsafe fn main() {
  let mut g = Global::new();
  g.terminal.color = make_color(Yellow, Blue);
  g.terminal.clear();

  g.terminal.write_string("================================================================================\x00");
  g.terminal.write_string("rv6 v.0.1 kernel is starting\n\x00");

  let vstart: *() = memory::get_end();
  let vend: *() = memory::P2V(4*1024*1024);

  g.terminal.write_num(vstart as int, 16, false);
  g.terminal.write_string("\n\x00");
  g.terminal.write_num(vend as int, 16, false);

  memory::kalloc::init_first(vstart, vend);
  g.terminal.write_string("\nKernel allocator setup (1/2)\x00");

  loop {};
}
