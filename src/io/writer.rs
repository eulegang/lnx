use crate::{Errno, ToErrno, Result, syscall::write};
use super::{fd, wfd};

pub trait Writer {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
}

impl Writer for fd {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes = write(self.fd, buf).to_errno()?;
        Ok(bytes as usize)
    }
}

impl Writer for wfd {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.0.write(buf)
    }
}

impl core::fmt::Write for fd {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let buf = s.as_bytes();

        let mut written = 0;

        while written < buf.len() {
            match self.write(&buf[written..]) {
                Ok(n) => written += n,
                Err(Errno::EINTR) => (),
                Err(_) => return Err(core::fmt::Error),
            }
        }

        Ok(())
    }
}

impl core::fmt::Write for wfd {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.0.write_str(s)
    }
}
