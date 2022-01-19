use super::{fd, rfd, wfd};
use crate::{
    syscall::{dup, dup2},
    Result, ToErrno,
};

pub trait Dup: Sized {
    fn dup(&self) -> Result<Self>;
    fn dup2(&self, target: i32) -> Result<Self>;
}

impl Dup for fd {
    fn dup(&self) -> Result<fd> {
        let fd = dup(self.fd).to_errno()? as i32;

        Ok(fd { fd })
    }

    fn dup2(&self, target: i32) -> Result<fd> {
        let fd = dup2(self.fd, target).to_errno()? as i32;

        Ok(fd { fd })
    }
}

impl Dup for rfd {
    fn dup(&self) -> Result<rfd> {
        self.0.dup().map(rfd)
    }

    fn dup2(&self, target: i32) -> Result<rfd> {
        self.0.dup2(target).map(rfd)
    }
}

impl Dup for wfd {
    fn dup(&self) -> Result<wfd> {
        self.0.dup().map(wfd)
    }

    fn dup2(&self, target: i32) -> Result<wfd> {
        self.0.dup2(target).map(wfd)
    }
}
