use console::vgaterm::*;

pub unsafe fn panic(s: &str)
{
  ::x86::cli();
  terminal.column = 0;
  terminal.row = 0;
  terminal.color = make_color(Black, Red);
  terminal.print_string("==================================[DON'T PANIC]=================================\x00");
  terminal.print_string("Kernel PANIC:\n\x00");
  terminal.print_string(s);
  terminal.print_string("\n================================================================================\x00");

  loop {}
}
