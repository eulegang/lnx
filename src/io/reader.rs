use crate::{SysErr, Result, syscall::read};
use super::{fd, rfd};

pub trait Reader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
}

impl Reader for fd {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes = read(self.fd, buf);

        if bytes == -1 {
            Err(SysErr::take())
        } else {
            Ok(bytes as usize)
        }
    }
}

impl Reader for rfd {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.0.read(buf)
    }
}

