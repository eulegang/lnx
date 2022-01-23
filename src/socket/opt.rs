use super::{builder, socket_addr, SocketType};
use crate::{
    syscall::{getsockopt, setsockopt},
    Result, ToErrno,
};

const SOL_SOCKET: u32 = 1;

pub(crate) trait SockOptValue: Sized {
    const SIZE: usize;

    fn marshal(&self, buf: &mut [u8]);
    fn unmarshal(buf: &[u8]) -> Self;
}

pub(crate) trait SockOpt {
    const NAME: u32;
    type Val: SockOptValue;
}

pub(crate) struct ReuseAddr {}
pub(crate) struct ReusePort {}
pub(crate) struct Broadcast {}
pub(crate) struct DontRoute {}
pub(crate) struct KeepAlive {}
pub(crate) struct Linger {
    pub(crate) enabled: u32,
    pub(crate) seconds: u32,
}

pub(crate) struct RcvBuf {}
pub(crate) struct SndBuf {}

impl SockOpt for ReuseAddr {
    const NAME: u32 = 2;
    type Val = bool;
}

impl SockOpt for ReusePort {
    const NAME: u32 = 15;
    type Val = bool;
}

impl SockOpt for Broadcast {
    const NAME: u32 = 6;
    type Val = bool;
}

impl SockOpt for DontRoute {
    const NAME: u32 = 5;
    type Val = bool;
}

impl SockOpt for KeepAlive {
    const NAME: u32 = 9;
    type Val = bool;
}

impl SockOpt for Linger {
    const NAME: u32 = 13;
    type Val = Linger;
}

impl SockOpt for RcvBuf {
    const NAME: u32 = 8;
    type Val = u32;
}

impl SockOpt for SndBuf {
    const NAME: u32 = 8;
    type Val = u32;
}

impl SockOptValue for bool {
    const SIZE: usize = 4;

    fn marshal(&self, buf: &mut [u8]) {
        1u32.marshal(buf)
    }

    fn unmarshal(buf: &[u8]) -> Self {
        buf[0] | buf[1] | buf[2] | buf[3] != 0
    }
}

impl SockOptValue for u32 {
    const SIZE: usize = 4;

    fn marshal(&self, buf: &mut [u8]) {
        (&mut buf[0..4]).copy_from_slice(&self.to_ne_bytes())
    }

    fn unmarshal(buf: &[u8]) -> u32 {
        u32::from_ne_bytes([buf[0], buf[1], buf[2], buf[3]])
    }
}

impl SockOptValue for Linger {
    const SIZE: usize = 8;

    fn marshal(&self, buf: &mut [u8]) {
        self.enabled.marshal(buf);
        self.seconds.marshal(&mut buf[4..]);
    }

    fn unmarshal(buf: &[u8]) -> Self {
        let enabled = u32::unmarshal(buf);
        let seconds = u32::unmarshal(&buf[4..]);

        Linger { enabled, seconds }
    }
}

impl<Addr: socket_addr, const TY: SocketType> builder<Addr, TY> {
    pub(crate) fn set_opt<S: SockOpt>(self, val: S::Val) -> Result<Self> {
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

    pub(crate) fn get_opt<S: SockOpt>(&self) -> Result<S::Val> {
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
