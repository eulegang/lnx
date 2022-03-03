//#![cfg_attr(not(test), no_std)]
#![no_std]
#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)] // not working properly
#![allow(incomplete_features)]

#[macro_export]
macro_rules! die {
    ($msg: literal, $($args: expr),*) => {{
        let _ = write!(::lnx::io::fd::stderr(), $msg, $($args),*);
        ::lnx::proc::Proc::exit(1);
    }}
}

mod err;
pub mod fs;
pub mod io;
pub mod mmap;
pub mod proc;

#[cfg(feature = "socket")]
pub mod socket;

pub(crate) mod konst;
pub(crate) mod syscall;

pub use err::Errno;
pub(crate) use err::ToErrno;

pub type Result<T> = core::result::Result<T, Errno>;
