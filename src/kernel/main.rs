#[link(name = "rv6kernel", vers = "0.1", author = "malte.harder")];

#[allow(ctypes)];
#[no_std];
#[no_core];

use vgaterm::*;

mod vgaterm;
mod zero;


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
  g.terminal.write_string("rv6 v.0.1 kernel is starting\x00");


  loop {};
}
