pub use memory::memory_c::*;

#[path="../../build/src/kernel/memory_c.rs"]
pub mod memory_c;

#[path="memory/kalloc.rs"]
pub mod kalloc;

#[path="memory/spinlock.rs"]
pub mod spinlock;

mod x86;

extern {
  #[fast_ffi]
  pub fn get_end() -> *();
}

pub unsafe fn memset(dst: *(), c: int, n: uint) {
  if (dst as int)%4 == 0 && n%4 == 0 {
    let cc = c & 0xFF;
    x86::stosl(dst, (cc << 24) | (cc << 16) | (cc << 8) | cc, n/4 as int);
  }
  else {
    x86::stosb(dst, c, n as int);
  }
}

pub fn ptr_add<T>(ptr: *T, op: uint) -> *T {
  ((ptr as uint) + op) as *T
}

pub fn ptr_inc<T>(ptr: &mut*T, op: uint) {
  *ptr = ((*ptr as uint) + op) as *T;
}

pub fn V2P<T>(a: *T) -> uint {
  (a as uint - KERNBASE)
}

pub fn P2V<T>(a: uint) -> *T {
  (a + KERNBASE) as *T
}

pub fn PGROUNDUP<T>(sz: uint) -> *T {
   (((sz)+PGSIZE-1) & !(PGSIZE-1)) as *T
}

pub fn PGROUNDDOWN<T>(a: uint) -> *T {
   (((a)) & !(PGSIZE-1)) as *T
}
