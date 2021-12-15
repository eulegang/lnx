#![no_std]
#![allow(non_camel_case_types)]

#![feature(asm)]
#![feature(asm_const)]

macro_rules! flag_impl {
    ($ty: ty) => {
        impl core::ops::BitOr for $ty {
            type Output = $ty;
            fn bitor(self, other: $ty) -> $ty {
                let flags = self.flags | other.flags;
                <$ty>::new(flags)
            }
        }

        impl core::ops::BitAnd for $ty {
            type Output = $ty;
            fn bitand(self, other: $ty) -> $ty {
                let flags = self.flags & other.flags;
                <$ty>::new(flags)
            }
        }

        impl core::ops::Not for $ty {
            type Output = $ty;
            fn not(self) -> $ty {
                <$ty>::new(!self.flags)
            }
        }
    }
}

mod err;
mod io;
mod mem;
pub(crate) mod string;

pub (crate) mod syscall;
pub (crate) mod konst;

pub use err::*;
pub use io::*;
pub use mem::*;
pub(crate) use string::*;
pub use string::CStr;

pub type Result<T> = core::result::Result<T, SysErr>;

pub mod prelude {
    pub use crate::string::CStr;
    pub use crate::Result;

    pub use crate::io::{Writer, Reader, fd, Close};
}
