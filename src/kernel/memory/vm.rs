use super::*;

struct KMap {
    virt: *(),
    phys_start: uint,
    phys_end: uint,
    perm: uint
}

pub static mut kmap: [KMap, ..4] = [
  KMap { virt: 0x0 as *(), phys_start: 0, phys_end: 0,  perm: 0},
  KMap { virt: 0x0 as *(), phys_start: 0, phys_end: 0,  perm: 0},
  KMap { virt: 0x0 as *(), phys_start: 0, phys_end: 0,  perm: 0},
  KMap { virt: 0x0 as *(), phys_start: 0, phys_end: 0,  perm: 0},
];

impl KMap {
  unsafe fn map(&self, pgdir: *()) -> int {
    let size = self.phys_end - self.phys_start;
    let mut pfirst = self.phys_start;
    let mut first: *() = PGROUNDDOWN(self.phys_start);
    let last: *() = PGROUNDDOWN(self.phys_end-1);
    let mut pte: *uint = null();
    loop {
      pte = walkpgdir(pgdir, first, true);
      if is_null(pte) {
        return -1;
      }
      if (*pte & PTE_P) != 0 {
        ::panic::panic("Remap\x00");
      }
      *(pte as *mut uint) = pfirst | self.perm | PTE_P;
      if first == last {
        break;
      }
      first = first + PGSIZE;
      pfirst += PGSIZE;
    }
    return 0;
  }
}

pub static mut kpgdir: *() = static_null;

pub unsafe fn walkpgdir(pgdir: *(), vaddr: *(), alloc: bool) -> *uint {
  return null();
}

pub unsafe fn setupkvm() -> *() {
  let pgdir: *() = ::memory::kalloc::alloc();
  if is_null(pgdir) {
    return null();
  }
  memset(pgdir, 0, PGSIZE);
  if P2Vi(PHYSTOP) > DEVSPACE {
    ::panic::panic("PHYSTOP too high\x00");
  }

  ::kutil::range(0,4, |i| {
    if kmap[i].map(pgdir) < 0 {
      ::panic::panic("Could not map page\x00");
    }
  });

  return pgdir;
}

pub unsafe fn alloc() {
  kmap[2].virt = get_data();
  kpgdir = setupkvm();
  switchkvm();
}

pub unsafe fn switchkvm() {
  ::x86::lcr3(V2P(kpgdir));
}