use crate::{SysErr, Result};

mod open;
mod pipe;
mod reader;
mod writer;
mod close;

pub use reader::Reader;
pub use writer::Writer;
pub use close::Close;
pub use pipe::Pipe;

pub use open::*;

#[link(name = "c")]
extern "C" {
    fn open(path: *const u8, flags: u32) -> i32;
    fn dup(fd: i32) -> i32;
    fn dup2(old: i32, new: i32) -> i32;
}

pub struct fd {
    fd: i32,
}

pub struct rfd(fd);
pub struct wfd(fd);

impl fd {
    pub const STDIN: rfd = rfd(fd { fd: 0 });
    pub const STDOUT: wfd = wfd(fd { fd: 1 });
    pub const STDERR: wfd = wfd(fd { fd: 2 });

    pub fn dup(&self) -> Result<fd> {
        let fd = unsafe { dup(self.fd) };

        fd { fd }.check()
    }

    pub fn dup_as(&self, other: fd) -> Result<()> {
        let check = unsafe { dup2(self.fd, other.fd) };

        if check == -1 {
            Err(SysErr::take())
        } else {
            Ok(())
        }
    }


    pub(crate) fn check(self) -> Result<fd> {
        if self.fd == -1 {
            Err(SysErr::take())
        } else {
            Ok(self)
        }
    }
}

