use crate::{Errno, Result, syscall::write};
use super::{fd, wfd};

pub trait Writer {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
}

impl Writer for fd {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes = Errno::new(write(self.fd, buf))?;
        Ok(bytes as usize)
    }
}

impl Writer for wfd {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.0.write(buf)
    }
}
