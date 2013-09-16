// Not ideal yet, would like to use rusts inline asm with inlining here
extern {
    #[fast_ffi]
    pub fn stosl(addr: *mut (), data: int, cnt: int) -> ();
    #[fast_ffi]
    pub fn stosb(addr: *mut (), data: int, cnt: int) -> ();
    #[fast_ffi]
    pub fn inb(port:u16) -> u8;
    #[fast_ffi]
    pub fn outb(port:u16, data: u8) -> ();
    #[fast_ffi]
    #[inline(always)]
    pub fn lcr3(val:uint) -> ();
}


pub unsafe fn cli() {
  asm!( "cli"
       :
       :
       :
       : "volatile"
       );
}

