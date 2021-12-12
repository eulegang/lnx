#![no_std]
#![allow(non_camel_case_types)]

mod err;
mod io;
mod mem;
pub(crate) mod string;

pub use err::*;
pub use io::*;
pub use mem::*;
pub(crate) use string::*;
pub use string::CStr;

pub mod prelude {
    pub use crate::string::CStr;
}
