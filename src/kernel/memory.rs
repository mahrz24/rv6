pub use memory::memory_c::*;
pub use memory::ptr::*;
use zero::Ord;

#[path="memory/memory_c.rs"]
mod memory_c;

#[path="memory/kalloc.rs"]
mod kalloc;

#[path="memory/vm.rs"]
mod vm;

#[path="memory/ptr.rs"]
mod ptr;

#[path="memory/spinlock.rs"]
mod spinlock;

extern {
  #[fast_ffi]
  pub fn get_end() -> *();
  #[fast_ffi]
  #[inline]
  pub fn get_data() -> *();
}



pub unsafe fn memcmp_str<T>(v1: *const T, v2: &str, n: uint) -> u8
{
  let (str_ptr, str_len): (*u8, uint) = transmute(v2);
  let mut n: uint = n;
  if str_len < n {
    n = str_len;
  }
  memcmp(v1, str_ptr, n)
}

pub unsafe fn memcmp<T,U>(v1: *const T, v2: *const U, n: uint) -> u8
{
  let mut s1: *u8 = v1 as *u8;
  let mut s2: *u8 = v2 as *u8;
  let mut n = n;
  while(n > 0){
    n-=1;
    if *s1 != *s2 {
      return *s1 - *s2;
    }
    s1 = s1 + 1;
    s2 = s2 + 1;
  }

  return 0;
}

/// Sets n bytes of memory pointed to by dst to c
pub unsafe fn memset<T>(dst: *mut T, c: u8, n: uint) -> *mut () {
  let dst: *mut () = dst as *mut ();
  if (dst as int)%4 == 0 && n%4 == 0 {
    let cc:int = c as int;
    ::x86::stosl(dst, (cc << 24) | (cc << 16) | (cc << 8) | cc, n/4 as int);
  }
  else {
    ::x86::stosb(dst, c as int, n as int);
  }
  dst
}

pub unsafe fn memmove<T,U>(dst: *mut T, src: *const U, n: uint) -> *mut T {
  let mut s: *u8 = src as *u8;
  let mut d: *mut u8 = dst as *mut u8;
  let mut n = n;

   if s < d && s + n > d {
     s = s + n;
     d = d + n;
     while n > 0 {
       s = s - 1;
       d = d - 1;
       *d = *s;
       n -= 1;
     }
   } else {
     while n > 0 {
  //     *d = *s;
       s = s + 1;
       d = d + 1;
       n -= 1;
     }
   }

  dst
}



pub fn V2P<T>(a: *const T) -> uint {
  (a as uint - KERNBASE)
}

pub fn V2Pi(a: uint) -> uint {
  (a - KERNBASE)
}

pub fn P2V<T>(a: uint) -> *T {
  (a + KERNBASE) as *T
}

pub fn mut_P2V<T>(a: uint) -> *mut T {
  (a + KERNBASE) as *mut T
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
