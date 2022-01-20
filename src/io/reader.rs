use super::{fd, rfd};
use crate::{syscall::read, Result, ToErrno};

pub trait Reader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
}

impl Reader for fd {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes = read(self.fd, buf.as_mut_ptr(), buf.len()).to_errno()?;
        Ok(bytes as usize)
    }
}

impl Reader for rfd {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.0.read(buf)
    }
}
