use super::*;
use super::spinlock::*;

struct RunList {
  next: *RunList
}

struct KMem {
  lock: Spinlock,
  use_lock: int,
  freelist: *RunList
}

pub static mut kmem: KMem = KMem { lock: Spinlock { locked: 0,
                                                    active : false,
                                                    name: 0x0 as *u8,
                                                    cpu: 0,
                                                    pcs: [0,0,0,0,0,0,0,0,0,0]
                                                  },
                                   use_lock: 0,
                                   freelist: 0x0 as *RunList
                                 };

pub unsafe fn init_first(vstart: *(), vend: *()) {
  kmem.lock.init("kmem");
  kmem.lock.active = false;
  freerange(vstart, vend);
}

unsafe fn freerange(vstart: *(), vend: *()) {
  let mut p: *() = PGROUNDUP(vstart as uint);
  while p + PGSIZE <= vend {
    free(p);
    p = p + PGSIZE;
  }
}

// Allocate a page of physical memory
pub unsafe fn alloc<T>() -> *mut T
{
  let mut r: *mut RunList = mut_null();

  do kmem.lock.protect {
    r = transmute(kmem.freelist);
    if r.is_not_null() {
      kmem.freelist = (*r).next;
    }
  }
  return transmute(r);
}

// Free a page of physical memory
unsafe fn free<T>(v: *T)
{
  let v = v as *mut ();

  if (v as uint) % PGSIZE > 0 || (v as uint) < (get_end() as uint) || V2P(v) >= PHYSTOP {
    ::panic::panic("kfree");
  }

  memset(v, 1, PGSIZE);

  do kmem.lock.protect {
    let r: *mut RunList = v as *mut RunList;
    (*r).next = kmem.freelist;
    kmem.freelist = r as *RunList;
  }
}

