#[link(name = "rv6kernel", vers = "0.1", author = "malte.harder")];

#[allow(ctypes)];
#[no_std];
#[no_core];

mod zero;

#[no_mangle]
pub unsafe fn main() {
    loop {};
}