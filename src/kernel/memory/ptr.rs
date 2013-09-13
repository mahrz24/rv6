// Raw pointer support for rust
// Parts taken from std::ptr from the rust std::lib
// with additions to allow pointer arithmetic
use zero::*;
pub use zero::transmute;

/// Calculate the offset from a pointer.
#[inline]
pub unsafe fn offset<T>(ptr: *T, count: uint) -> *T {
    (ptr as uint + (count as uint) * size_of::<T>()) as *T
}

#[inline]
pub unsafe fn mut_offset<T>(ptr: *mut T, count: uint) -> *mut T {
    (ptr as uint + (count as uint) * size_of::<T>()) as *mut T
}

/// Calculate the negative offset from a pointer.
#[inline]
pub unsafe fn offset_back<T>(ptr: *T, count: uint) -> *T {
    (ptr as uint - (count as uint) * size_of::<T>()) as *T
}

#[inline]
pub unsafe fn mut_offset_back<T>(ptr: *mut T, count: uint) -> *mut T {
    (ptr as uint - (count as uint) * size_of::<T>()) as *mut T
}

pub static static_null: *() = 0x0 as *();

#[inline(always)]
pub fn null<T>() -> *T { 0x0 as *T }

#[inline(always)]
pub fn mut_null<T>() -> *mut T { 0x0 as *mut T }

#[inline]
pub fn is_null<T>(ptr: *const T) -> bool { ptr == null() }

#[inline]
pub fn is_not_null<T>(ptr: *const T) -> bool { !is_null(ptr) }

pub trait RawPtr<T> {
    fn is_null(&self) -> bool;
    fn is_not_null(&self) -> bool;
    fn offset(&self, count: uint) -> Self;
    fn offset_back(&self, count: uint) -> Self;
}

impl<T> RawPtr<T> for *T {
    /// Returns true if the pointer is equal to the null pointer.
    #[inline]
    fn is_null(&self) -> bool { is_null(*self) }

    /// Returns true if the pointer is not equal to the null pointer.
    #[inline]
    fn is_not_null(&self) -> bool { is_not_null(*self) }

    /// Calculates the offset from a pointer.
    #[inline]
    fn offset(&self, count: uint) -> *T { unsafe{ offset(*self, count) } }

    /// Calculates the negative offset from a pointer.
    #[inline]
    fn offset_back(&self, count: uint) -> *T { unsafe{ offset_back(*self, count) } }
}

/// Extension methods for mutable pointers
impl<T> RawPtr<T> for *mut T {
    /// Returns true if the pointer is equal to the null pointer.
    #[inline]
    fn is_null(&self) -> bool { is_null(*self) }

    /// Returns true if the pointer is not equal to the null pointer.
    #[inline]
    fn is_not_null(&self) -> bool { is_not_null(*self) }

    /// Calculates the offset from a mutable pointer.
    #[inline]
    fn offset(&self, count: uint) -> *mut T { unsafe{ mut_offset(*self, count) } }

     /// Calculates the negative offset from a pointer.
    #[inline]
    fn offset_back(&self, count: uint) -> *mut T { unsafe{ mut_offset_back(*self, count) } }
}

// Equality for pointers
impl<T> Eq for *const T {
    #[inline]
    fn eq(&self, other: &*const T) -> bool {
        (*self as uint) == (*other as uint)
    }
    #[inline]
    fn ne(&self, other: &*const T) -> bool { !self.eq(other) }
}

// Comparison for pointers
impl<T> Ord for *const T {
    #[inline]
    fn lt(&self, other: &*const T) -> bool {
        (*self as uint) < (*other as uint)
    }
    #[inline]
    fn le(&self, other: &*const T) -> bool {
        (*self as uint) <= (*other as uint)
    }
    #[inline]
    fn ge(&self, other: &*const T) -> bool {
        (*self as uint) >= (*other as uint)
    }
    #[inline]
    fn gt(&self, other: &*const T) -> bool {
        (*self as uint) > (*other as uint)
    }
}

// Pointer arithmetic
impl<T> Add<uint, *T> for *T {
    #[inline]
    fn add(&self, rhs: &uint) -> *T {
        self.offset(*rhs)
    }
}

impl<T> Sub<uint, *T> for *T {
    #[inline]
    fn sub(&self, rhs: &uint) -> *T {
        self.offset_back(*rhs)
    }
}


impl<T> Add<uint, *mut T> for *mut T {
    #[inline]
    fn add(&self, rhs: &uint) -> *mut T {
        self.offset(*rhs)
    }
}

impl<T> Sub<uint, *mut T> for *mut T {
    #[inline]
    fn sub(&self, rhs: &uint) -> *mut T {
        self.offset_back(*rhs)
    }
}
