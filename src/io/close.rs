use crate::{SysErr, Result, syscall::close};
use super::{fd, wfd, rfd};

pub trait Close {
    fn close(self) -> Result<()>;
}

impl Close for fd {
    fn close(self) -> Result<()> {
        let result = close(self.fd);
        core::mem::forget(self);
        
        match result {
            0 => Ok(()),
            _ => Err(SysErr::take()),
        }
    }
}

impl Close for rfd {
    fn close(self) -> Result<()> {
        self.0.close()
    }
}

impl Close for wfd {
    fn close(self) -> Result<()> {
        self.0.close()
    }
}

impl Drop for fd {
    fn drop(&mut self) {
        close(self.fd);
    }
}
