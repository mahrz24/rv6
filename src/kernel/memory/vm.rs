use super::*;
use console::vgaterm::*;

struct KMap {
    virt: *(),
    phys_start: uint,
    phys_end: uint,
    perm: uint
}

type Pde = uint;
type Pte = uint;

pub static mut kmap: [KMap, ..4] = [
  KMap { virt: 0x0 as *(), phys_start: 0, phys_end: 0,  perm: 0},
  KMap { virt: 0x0 as *(), phys_start: 0, phys_end: 0,  perm: 0},
  KMap { virt: 0x0 as *(), phys_start: 0, phys_end: 0,  perm: 0},
  KMap { virt: 0x0 as *(), phys_start: 0, phys_end: 0,  perm: 0},
];

impl KMap {
  unsafe fn map(&self, pgdir: *mut Pde) -> int {
    let size = self.phys_end - self.phys_start;
    let mut pfirst = self.phys_start;
    let mut vfirst: *() = PGROUNDDOWN(self.virt as uint);
    let vlast: *() = PGROUNDDOWN(self.virt as uint + size - 1);
    let mut pte: *mut Pte = mut_null();
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
  terminal.print_num(pgdir as int, 16, false);
  if is_null(pgdir) {
    return null();
  }
  memset(pgdir, 0, PGSIZE);
  if P2Vi(PHYSTOP) > DEVSPACE {
    ::panic::panic("PHYSTOP too high");
  }

  ::kutil::range(0,4, |i| {
    terminal.print_string("Mapping pgtable\n");
    if kmap[i].map(pgdir) < 0 {
      ::panic::panic("Could not map page");
    }
  });

  return transmute(pgdir);
}

pub unsafe fn alloc() {

  kmap[0].virt = transmute(KERNBASE);
  kmap[1].virt = transmute(KERNLINK);
  kmap[2].virt = get_data();
  kmap[3].virt = transmute(DEVSPACE);
  kmap[0].phys_start = 0;
  kmap[1].phys_start = V2Pi(KERNLINK);
  kmap[2].phys_start = V2P(get_data());
  kmap[3].phys_start = DEVSPACE;
  kmap[0].phys_end = EXTMEM;
  kmap[1].phys_end = V2P(get_data());
  kmap[2].phys_end = PHYSTOP;
  kmap[3].phys_end = 0;
  kmap[0].perm = PTE_W;
  kmap[1].perm = 0;
  kmap[2].perm = PTE_W;
  kmap[3].perm = PTE_W;

  kpgdir = setupkvm();
  switchkvm();
}

pub unsafe fn switchkvm() {
  terminal.print_num(kpgdir as int, 16, false);
  //::x86::lcr3(V2P(kpgdir));
}