
struct Spinlock {
  locked: uint,
  active: bool,
  // For debugging purposes
  name: *u8,
  cpu: uint,
  pcs: [uint, ..10]
}

impl Spinlock {
  pub fn init(&mut self, name: &str)
  {
    self.active = true;
    self.locked = 0;
    self.name = &(name[0]) as *u8;
    self.cpu = 0;
  }

  pub fn protect(self, op: &fn()) {
    if(self.active) {
      self.acquire();
    }

    op();

    if(self.active) {
      self.release();
    }
  }

  pub fn acquire(self) {

  }

  pub fn release(self) {

  }
}