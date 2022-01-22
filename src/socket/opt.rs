use super::{builder, socket_addr};
use crate::{
    syscall::{getsockopt, setsockopt},
    Result, ToErrno,
};

const SOL_SOCKET: u32 = 1;

pub trait SockOptValue: Sized {
    const SIZE: usize;

    fn marshal(&self, buf: &mut [u8]);
    fn unmarshal(buf: &[u8]) -> Self;
}

pub trait SockOpt {
    const NAME: u32;
    type Val: SockOptValue;
}

pub struct ReuseAddr {}
pub struct ReusePort {}

impl SockOpt for ReuseAddr {
    const NAME: u32 = 2;
    type Val = bool;
}

impl SockOpt for ReusePort {
    const NAME: u32 = 15;
    type Val = bool;
}

impl SockOptValue for bool {
    const SIZE: usize = 4;

    fn marshal(&self, buf: &mut [u8]) {
        (&mut buf[0..4]).copy_from_slice(&1u32.to_ne_bytes())
    }

    fn unmarshal(buf: &[u8]) -> Self {
        buf[0] | buf[1] | buf[2] | buf[3] != 0
    }
}

impl<Addr: socket_addr> builder<Addr> {
    pub fn set_opt<S: SockOpt>(self, val: S::Val) -> Result<builder<Addr>> {
        let mut buf = [0u8; 16];
        val.marshal(&mut buf);

        setsockopt(
            self.fd.fd,
            SOL_SOCKET,
            S::NAME,
            &buf as *const u8,
            S::Val::SIZE,
        )
        .to_errno()?;

        Ok(self)
    }

    pub fn get_opt<S: SockOpt>(&self) -> Result<S::Val> {
        let mut buf = [0u8; 16];
        let mut len = 0usize;

        getsockopt(
            self.fd.fd,
            SOL_SOCKET,
            S::NAME,
            &mut buf as *mut u8,
            &mut len as *mut usize,
        )
        .to_errno()?;

        Ok(S::Val::unmarshal(&buf))
    }
}
