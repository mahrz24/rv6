pub use memory::memory_c::*;

#[path="memory/memory_c.rs"]
mod memory_c;

#[path="memory/kalloc.rs"]
mod kalloc;

#[path="memory/vm.rs"]
mod vm;


#[path="memory/spinlock.rs"]
mod spinlock;

pub static cnull_ptr: *() = 0x0 as *();

pub fn null_ptr<T>() -> *T {
  return 0x0 as *T;
}

pub fn mnull_ptr<T>() -> *mut T {
  return 0x0 as *mut T;
}


pub fn is_null<T>(p: *T) -> bool {
  return p as uint == 0;
}

pub fn mis_null<T>(p: *mut T) -> bool {
  return p as uint == 0;
}

extern {
  #[fast_ffi]
  pub fn get_end() -> *();
  #[fast_ffi]
  pub fn get_data() -> *();
}

pub unsafe fn memset(dst: *(), c: u8, n: uint) -> *() {
  if (dst as int)%4 == 0 && n%4 == 0 {
    let cc:int = c as int;
    ::x86::stosl(dst, (cc << 24) | (cc << 16) | (cc << 8) | cc, n/4 as int);
  }
  else {
    ::x86::stosb(dst, c as int, n as int);
  }
  dst
}

pub unsafe fn memmove(dst: *mut (), src: *(), n: uint) -> *mut () {
  let mut s: *u8 = src as *u8;
  let mut d: *mut u8 = dst as *mut u8;
  let mut n = n;

  if (s as uint) < (d as uint) && (s as uint) + n > (d as uint) {
    s = ptr_add(s,n);
    d = ptr_addm(d,n);
    while n > 0 {
      s = ptr_add(s,-1);
      d = ptr_addm(d,-1);
      *d = *s;
      n -= 1;
    }
  } else {
    while n > 0 {
      *d = *s;
      s = ptr_add(s,1);
      d = ptr_addm(d,1);
      n -= 1;
    }
  }

  dst
}

pub fn cast<T,U>(ptr: *T) -> *U {
  ptr as *U
}

pub fn ptr_m2i<T,U>(ptr: *mut T) -> *U {
  ptr as *U
}

pub fn ptr_i2m<T,U>(ptr: *T) -> *mut U {
  ptr as *mut U
}


pub fn ptr_addm<T>(ptr: *mut T, op: uint) -> *mut T {
  ((ptr as uint) + op) as *mut T
}

pub fn ptr_add<T>(ptr: *T, op: uint) -> *T {
  ((ptr as uint) + op) as *T
}

pub fn ptr_inc<T>(ptr: &mut*T, op: uint) {
  *ptr = ((*ptr as uint) + op) as *T;
}

pub fn ptr_eq<T,U>(ptr: *T, ptrb: *U) -> bool {
  (ptr as uint) == (ptrb as uint)
}

pub fn V2P<T>(a: *T) -> uint {
  (a as uint - KERNBASE)
}

pub fn P2V<T>(a: uint) -> *T {
  (a + KERNBASE) as *T
}

pub fn P2Vi(a: uint) -> uint {
  (a + KERNBASE)
}

pub fn PGROUNDUP<T>(sz: uint) -> *T {
   (((sz)+PGSIZE-1) & !(PGSIZE-1)) as *T
}

pub fn PGROUNDDOWN<T>(a: uint) -> *T {
   (((a)) & !(PGSIZE-1)) as *T
}
