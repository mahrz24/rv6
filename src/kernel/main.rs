#[link(name = "rv6kernel", vers = "0.1", author = "malte.harder")];

#[allow(ctypes)];
#[no_std];
#[no_core];

use console::vgaterm::*;


mod zero;
pub mod panic;
pub mod console;
pub mod macros;
pub mod memory;
pub mod x86;
pub mod kutil;

#[no_mangle]
pub unsafe fn main() {
  terminal = VGATerminal::new();

  terminal.color = make_color(Yellow, Blue);
  terminal.clear();

  terminal.print_string("================================================================================");
  terminal.print_string("rv6 v.0.1 kernel is starting\n");

  let vstart: *() = memory::get_end();
  let vend: *() = memory::P2V(4*1024*1024);

  kfmt!(terminal, "Kernel initial bounds [%x] to [%x]\n", (vstart as int), (vend as int))

  memory::kalloc::init_first(vstart, vend);
  terminal.print_string("Kernel allocator setup (1/2)\n");
  memory::vm::alloc();
  terminal.print_string("Allocated kernel page table\n");

  loop {};
}
