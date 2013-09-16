use zero::*;

pub enum VGAColor {
    Black       = 0,
    Blue        = 1,
    Green       = 2,
    Cyan        = 3,
    Red         = 4,
    Pink        = 5,
    Brown       = 6,
    LightGray   = 7,
    DarkGray    = 8,
    LightBlue   = 9,
    LightGreen  = 10,
    LightCyan   = 11,
    LightRed    = 12,
    LightPink   = 13,
    Yellow      = 14,
    White       = 15,
}

struct VGATextColor(u8);
struct VGAEntry(u16);

pub static VGA_WIDTH: uint = 80;
pub static VGA_HEIGHT: uint = 25;

pub struct VGATerminal {
  row: uint,
  column: uint,
  color: VGATextColor,
  buffer: *mut u16
}

pub enum Format {
  Integer(int, bool),
  String
}


pub trait Printable {
  unsafe fn print(&self, &mut VGATerminal, Format);
}

impl Printable for int {
  unsafe fn print(&self, term:&mut VGATerminal, fmt:Format) {
    match fmt {
      Integer(base, sign) => term.print_num(*self, base, sign),
      _ => ::panic::panic("wrong print format")
    }
  }
}

impl Printable for uint {
  unsafe fn print(&self, term:&mut VGATerminal, fmt:Format) {
    match fmt {
      Integer(base, sign) => term.print_num(*self as int, base, sign),
      _ => ::panic::panic("wrong print format")
    }
  }
}

impl Printable for u8 {
  unsafe fn print(&self, term:&mut VGATerminal, fmt:Format) {
    match fmt {
      Integer(base, sign) => term.print_num(*self as int, base, sign),
      _ => ::panic::panic("wrong print format")
    }
  }
}

impl<T> Printable for *T {
  unsafe fn print(&self, term:&mut VGATerminal, fmt:Format) {
    match fmt {
      Integer(base, sign) => term.print_num(*self as int, base, sign),
      _ => ::panic::panic("wrong print format")
    }
  }
}

impl<T> Printable for *mut T {
  unsafe fn print(&self, term:&mut VGATerminal, fmt:Format) {
    match fmt {
      Integer(base, sign) => term.print_num(*self as int, base, sign),
      _ => ::panic::panic("wrong print format")
    }
  }
}

impl Printable for &'static str {
  unsafe fn print(&self, term:&mut VGATerminal, fmt:Format) {
    match fmt {
      String => term.print_string(*self),
      _ => ::panic::panic("wrong print format")
    }
  }
}

pub static mut terminal: VGATerminal = VGATerminal { row: 0,
                                                     column: 0,
                                                     color: VGATextColor(0),
                                                     buffer: 0x0 as *mut u16
                                                   };

pub fn make_color(fg: VGAColor, bg: VGAColor) -> VGATextColor {
  VGATextColor((fg as u8) | (bg as u8) << 4)
}

pub fn make_vgaentry(c: u8, color: VGATextColor) -> VGAEntry {
  let c16: u16 = c as u16;
  let color16: u16 = *color as u16;
  VGAEntry(c16 | color16 << 8)
}

impl VGATerminal {
  pub fn new() -> VGATerminal {
    let term = VGATerminal { row: 0,
                  column: 0,
                  color: make_color(White, Black),
                  buffer: 0xb8000 as *mut u16
                };
    term
  }

  unsafe fn set_entry(self, pos: uint, entry: VGAEntry) {
    *((self.buffer[pos]) as *mut u16) = *entry;
  }

  unsafe fn set_cursor(self, pos: uint) {
    let CRTPORT:u16 = 0x3d4;
    ::x86::outb(CRTPORT, 14);
    ::x86::outb(CRTPORT+1, (pos>>8) as u8);
    ::x86::outb(CRTPORT, 15);
    ::x86::outb(CRTPORT+1, pos as u8);
  }

  unsafe fn update_cursor(self) {
    self.set_cursor(self.column + VGA_WIDTH*self.row);
  }

  unsafe fn set_entry_at(self, row: uint, col: uint, entry: VGAEntry) {
    self.set_entry(col + VGA_WIDTH*row, entry);
  }

  unsafe fn set_entry_cur(self, entry: VGAEntry) {
    self.set_entry(self.column + VGA_WIDTH*self.row, entry);
  }

  pub unsafe fn put_char(&mut self, c: u8) {
    if c == ('\n' as u8) {
      self.column = 0;
      self.row += 1;
    }
    else {
      self.set_entry_at(self.row, self.column, make_vgaentry(c, self.color));
      self.column+=1;
      if self.column >= VGA_WIDTH
      {
        self.column = 0;
        self.row += 1;
      }
    }


    if self.row == VGA_HEIGHT
    {
      ::memory::memmove(self.buffer, self.buffer + 80, 2*24*80);
      ::kutil::range(VGA_WIDTH*24, VGA_WIDTH*25, |i| {
        self.set_entry(i,make_vgaentry(' ' as u8, self.color));
      });
      self.row -= 1;
    }

    self.update_cursor();
  }

  pub unsafe fn print_num(&mut self, num: int, base: int, s: bool) {
    let mut sign = s;
    let digits = "0123456789abcdef";
    let mut buf:[u8, ..16] = [0 as u8, ..16];
    let mut i: int;
    let mut x: uint;

    if sign && (num < 0) {
      sign = true;
      x = -num as uint;
    }
    else {
      sign = false;
      x = num as uint;
    }

    i = 0;
    loop {
      buf[i] = digits[x % (base as uint)];
      i+=1;
      x /= base as uint;
      if x == 0 {
        break;
      }
    }

    if sign {
      buf[i] = '-' as u8;
      i+=1;
    }

    if(base==16)
    {
      self.put_char('0' as u8);
      self.put_char('x' as u8);
    }

    i -= 1;
    while(i >= 0) {
      self.put_char(buf[i]);
      i -= 1;
    }
  }

  pub unsafe fn print_string(&mut self, string: &str) {
    let (str_ptr, str_len): (*u8, uint) = transmute(string);
    let mut len = 0;
    while len < str_len {
      self.put_char(*str_ptr[len]);
      len+=1;
    }
  }

  pub unsafe fn print_format(&mut self, string: &str,
    fmt: &fn(&mut VGATerminal, uint, Format)) {
    let (str_ptr, str_len): (*u8, uint) = transmute(string);
    let mut len = 0;
    let mut arg = 0;
    while len < str_len {
      let mut c:u8 = *str_ptr[len];
      if c != '%' as u8 {
        self.put_char(c);
      }
      else {
        len+=1;
        c = *str_ptr[len];
        if len < str_len {
          match c as char {
            'd' => fmt(self, arg, Integer(10, true)),
            'x' | 'p' => fmt(self, arg, Integer(16, false)),
            's' => fmt(self, arg, String),
            _ => {
              self.put_char('%' as u8);
              self.put_char(c);
            }
          }
          arg+=1;
        }
      }
      len+=1;
    }
  }

  pub unsafe fn clear(self) {
    ::kutil::range(0, VGA_WIDTH*VGA_HEIGHT, |i| {
      self.set_entry(i,make_vgaentry(' ' as u8, self.color));
    });
  }
}
