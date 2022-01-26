//#![cfg_attr(not(test), no_std)]
#![no_std]
#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)] // not working properly
#![allow(incomplete_features)]
#![feature(adt_const_params)]

#[cfg(feature = "startup")]
#[macro_export]
macro_rules! setup_main {
    () => {
        #[no_mangle]
        #[naked]
        unsafe fn _start() {
            core::arch::asm!("mov rdi, rsp", "call main", options(noreturn));
        }
    };
}

#[macro_export]
macro_rules! die {
    ($msg: literal, $($args: expr),*) => {{
        let _ = write!(::lnx::io::fd::stderr(), $msg, $($args),*);
        ::lnx::proc::Proc::exit(1);
    }}
}

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
    };
}

mod err;
pub mod fs;
pub mod io;
pub mod mmap;
pub mod proc;

#[cfg(feature = "startup")]
pub mod start;

#[cfg(feature = "socket")]
pub mod socket;

pub(crate) mod konst;
pub(crate) mod syscall;

pub use err::Errno;
pub(crate) use err::ToErrno;

pub type Result<T> = core::result::Result<T, Errno>;
