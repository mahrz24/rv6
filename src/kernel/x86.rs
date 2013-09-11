// Not ideal yet, would like to use rusts inline asm with inlining here
extern {
    #[fast_ffi]
    pub fn stosl(addr: *(), data: int, cnt: int) -> ();
    #[fast_ffi]
    pub fn stosb(addr: *(), data: int, cnt: int) -> ();
}
