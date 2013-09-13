use console::vgaterm::*;

pub fn panic(s: &str)
{
  unsafe
  {
    ::x86::cli();
    terminal.column = 0;
    terminal.row = 0;
    terminal.color = make_color(Black, Red);
    terminal.print_string("==================================[DON'T PANIC]=================================");
    terminal.print_string("Kernel PANIC:\n");
    terminal.print_string(s);
    terminal.print_string("\n================================================================================");
  }
  loop {}
}
