use crate::{SysErr, SysErrOptExt};

mod open;

pub use open::*;

#[link(name = "c")]
extern "C" {
    fn open(path: *const u8, flags: u32) -> i32;
    fn write(fd: i32, buf: *const u8, len: u64) -> isize;
    fn read(fd: i32, buf: *mut u8, len: u64) -> isize;
    fn close(fd: i32) -> i32;
    fn dup(fd: i32) -> i32;
    fn dup2(old: i32, new: i32) -> i32;
}

pub struct fd {
    fd: i32,
}

impl fd {
    pub const STDIN: fd = fd { fd: 0 };
    pub const STDOUT: fd = fd { fd: 1 };
    pub const STDERR: fd = fd { fd: 2 };

    pub fn read(&self, buf: &mut [u8]) -> Result<usize, SysErr> {
        let bytes = unsafe { read(self.fd, buf.as_mut_ptr(), buf.len() as u64) };

        if bytes == -1 {
            Err(SysErr::take())
        } else {
            Ok(bytes as usize)
        }
    }

    pub fn write(&self, buf: &[u8]) -> Result<usize, SysErr> {
        let bytes = unsafe { write(self.fd, buf.as_ptr(), buf.len() as u64) };

        if bytes == -1 {
            Err(SysErr::take())
        } else {
            Ok(bytes as usize)
        }
    }

    pub fn dup(&self) -> Result<fd, SysErr> {
        let fd = unsafe { dup(self.fd) };

        fd { fd }.check()
    }

    pub fn dup_as(&self, other: fd) -> Result<(), SysErr> {
        let check = unsafe { dup2(self.fd, other.fd) };

        if check == -1 {
            Err(SysErr::take())
        } else {
            Ok(())
        }
    }

    pub fn close(self) -> Result<(), SysErr> {
        let result = unsafe { close(self.fd) };

        SysErr::check_syscall(result).check(|| ())
    }

    pub(crate) fn check(self) -> Result<fd, SysErr> {
        if self.fd == -1 {
            Err(SysErr::take())
        } else {
            Ok(self)
        }
    }
}

impl Drop for fd {
    fn drop(&mut self) {
        unsafe {
            close(self.fd);
        }
    }
}
