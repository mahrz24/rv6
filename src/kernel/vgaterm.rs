mod kutil;

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

static VGA_WIDTH: uint = 80;
static VGA_HEIGHT: uint = 25;

pub struct VGATerminal {
  row: uint,
  column: uint,
  color: VGATextColor,
  buffer: *mut u16
}

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
    *((self.buffer as uint + pos * 2) as *mut u16) = *entry;
  }

  unsafe fn set_entry_at(self, x: uint, y: uint, entry: VGAEntry) {
    self.set_entry(y + VGA_WIDTH*x, entry);
  }

  pub unsafe fn put_char(&mut self, c: u8)
  {
    self.set_entry_at(self.row, self.column, make_vgaentry(c, self.color));
    self.column+=1;
    if self.column >= VGA_WIDTH
    {
      self.column = 0;
      self.row += 1;
      if self.row >= VGA_HEIGHT-1
      {
        self.row = 0;
      }
    }
  }

  pub unsafe fn write_string(&mut self, data: &str)
  {
    let mut len = 0;
    while data[len] != 0 {
      if data[len] == '\n' as u8 {
        self.column = 0;
        self.row += 1;
        if self.row >= VGA_HEIGHT-1
        {
          self.row = 0;
        }
      }
      else {
       self.put_char(data[len]);
      }
      len+=1;
    }
  }

  pub unsafe fn clear(self) {
    kutil::range(0, VGA_WIDTH*VGA_HEIGHT, |i| {
      self.set_entry(i,make_vgaentry(' ' as u8, self.color));
    });
  }
}
