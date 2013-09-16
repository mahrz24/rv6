use super::*;
use console::vgaterm::*;

struct KMap {
    virt: *(),
    phys_start: uint,
    phys_end: uint,
    perm: uint
}

pub type Pde = uint;
pub type Pte = uint;

pub static nkmaps:uint = 5;
pub static null_map:KMap =  KMap { virt: 0x0 as *(), phys_start: 0, phys_end: 0,  perm: 0};

pub static mut kmap: [KMap, ..nkmaps] = [
  null_map, null_map, null_map, null_map, null_map
];

impl KMap {
  unsafe fn map(&self, pgdir: *mut Pde) -> int {
    let size = self.phys_end - self.phys_start;
    let mut pfirst = self.phys_start;
    let mut vfirst: *() = PGROUNDDOWN(self.virt as uint);
    let vlast: *() = PGROUNDDOWN(self.virt as uint + size - 1);
    let mut pte: *mut Pte;
    loop {
      pte = walkpgdir(pgdir, vfirst, true);

      if is_null(pte) {
        return -1;
      }
      if *pte & PTE_P != 0 {
        ::panic::panic("Remap");
      }
      *(pte) = pfirst | self.perm | PTE_P;
      if vfirst == vlast {
        break;
      }
      vfirst = vfirst + PGSIZE;
      pfirst += PGSIZE;
    }
    return 0;
  }
}

pub static mut kpgdir: *Pde = static_null as *Pde;

// Page directory index
fn PDX<T>(vaddr: *T) -> uint { (vaddr as uint >> PDXSHIFT) & 0x3FF }
// Page table index
fn PTX<T>(vaddr: *T) -> uint { (vaddr as uint >> PTXSHIFT) & 0x3FF }

fn PTE_ADDR(pte: Pte) -> uint { pte & !0xFFF }

pub unsafe fn walkpgdir(pgdir: *mut Pde, vaddr: *(), alloc: bool) -> *mut Pde {
  let pde: *mut Pde = pgdir[PDX(vaddr)];
  let pgtab: *mut Pte;

  if *pde & PTE_P != 0 {
    pgtab = mut_P2V(PTE_ADDR(*pde));
  } else {
    if(!alloc) {
      return mut_null();
    } else {
      pgtab = ::memory::kalloc::alloc();

      if(pgtab.is_null()) {
        return mut_null();
      }
      memset(pgtab, 0, PGSIZE);

      *pde = V2P(pgtab) | PTE_P | PTE_W | PTE_U;
    }
  }

  return pgtab[PTX(vaddr)];
}

pub unsafe fn setupkvm() -> *Pde {
  let pgdir: *mut Pde = ::memory::kalloc::alloc();
  if is_null(pgdir) {
    return null();
  }
  memset(pgdir, 0, PGSIZE);
  if P2Vi(PHYSTOP) > DEVSPACE {
    ::panic::panic("PHYSTOP too high");
  }

  ::kutil::range(0,nkmaps, |i| {
    kfmt!(terminal, "Page table %d: %x %x %x %x \n", i, kmap[i].virt, kmap[i].phys_start, kmap[i].phys_end, kmap[i].perm );

    if kmap[i].map(pgdir) < 0 {
      ::panic::panic("Could not map page");
    }
  });

  return transmute(pgdir);
}

pub unsafe fn alloc() {

  // We need this extra mapping, to refrain rusts
  // stack guard comparison with [gs:0x30] from triggering
  // a page fault.
  kmap[0] = KMap { virt: null(),
                   phys_start: 0,
                   phys_end: PGSIZE,
                   perm: PTE_W};
  kmap[1] = KMap { virt: transmute(KERNBASE),
                   phys_start: 0,
                   phys_end: EXTMEM,
                   perm: PTE_W};
  kmap[2] = KMap { virt: transmute(KERNLINK),
                   phys_start: V2Pi(KERNLINK),
                   phys_end: V2P(get_data()),
                   perm: 0};
  kmap[3] = KMap { virt: get_data(),
                   phys_start: V2P(get_data()),
                   phys_end: PHYSTOP,
                   perm: PTE_W};
  kmap[4] = KMap { virt: transmute(DEVSPACE),
                   phys_start:DEVSPACE,
                   phys_end: 0,
                   perm: PTE_W};


  kpgdir = setupkvm();
  switchkvm();

}

pub unsafe fn switchkvm() {
  // ::kutil::range(0,1024, |i| {
  //   if  *kpgdir[i] != 0 {
  //     kfmt!(terminal, "Pde in pgdir %x: %x %x \n", kpgdir, i, *kpgdir[i]);
  //     let pgtab: *mut Pte = mut_P2V(PTE_ADDR(*kpgdir[i]));
  //     ::kutil::range(0,1024, |i| {
  //       if  *pgtab[i] != 0 {
  //         kfmt!(terminal, "Pte in pgtab %x: %x %x \n", pgtab, i, *pgtab[i]);

  //       }
  //     });
  //  }
  // });
  kfmt!(terminal, "Table at: %x \n", V2P(kpgdir));
  ::x86::lcr3(V2P(kpgdir));
  terminal.buffer = mut_P2V(terminal.buffer as uint);
}