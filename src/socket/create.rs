use core::marker::PhantomData;

use crate::err::ToErrno;
use crate::io::fd;
use crate::socket::addr::socket_addr;
use crate::syscall::{bind, connect, socket as sys_socket};
use crate::Result;

use super::opt::{Broadcast, DontRoute, KeepAlive, Linger, RcvBuf, ReuseAddr, ReusePort, SndBuf};
use super::{listen, socket};

pub trait SocketType {
    const TYPE: u32;
}

pub struct Stream;
pub struct Datagram;

impl SocketType for Stream {
    const TYPE: u32 = 1;
}

impl SocketType for Datagram {
    const TYPE: u32 = 2;
}

pub struct builder<Addr: socket_addr, TY: SocketType> {
    pub(crate) fd: fd,
    _addr: PhantomData<(Addr, TY)>,
}

impl<Addr: socket_addr, TY: SocketType> builder<Addr, TY> {
    pub fn new() -> Result<Self> {
        let sock = sys_socket(Addr::FAMILY as u32, TY::TYPE as u32, 0);
        let fd = sock.to_errno()? as i32;
        let fd = fd { fd };

        let _addr = PhantomData;

        Ok(builder { fd, _addr })
    }
}

impl<Addr: socket_addr, TY: SocketType> builder<Addr, TY> {
    pub fn listen(self, addr: Addr) -> Result<listen<Addr>> {
        let fd = self.fd;
        bind(fd.fd, (&addr as *const Addr).cast(), Addr::SIZE).to_errno()?;
        listen(fd.fd, 10).to_errno()?;

        let _address = PhantomData;
        Ok(listen { fd, _address })
    }

    pub fn connect(self, addr: Addr) -> Result<socket> {
        let fd = self.fd;

        connect(fd.fd, (&addr as *const Addr).cast(), Addr::SIZE).to_errno()?;

        Ok(socket { fd })
    }

    pub fn reuse_addr(self, enabled: bool) -> Result<Self> {
        self.set_opt::<ReuseAddr>(enabled)
    }

    pub fn reuse_port(self, enabled: bool) -> Result<Self> {
        self.set_opt::<ReusePort>(enabled)
    }

    pub fn dont_route(self, enabled: bool) -> Result<Self> {
        self.set_opt::<DontRoute>(enabled)
    }

    pub fn linger(self, seconds: Option<u32>) -> Result<Self> {
        match seconds {
            Some(seconds) => {
                let enabled = 1;
                self.set_opt::<Linger>(Linger { enabled, seconds })
            }

            None => {
                let enabled = 0;
                let seconds = 0;
                self.set_opt::<Linger>(Linger { enabled, seconds })
            }
        }
    }

    pub fn recv_buf(self, size: u32) -> Result<Self> {
        self.set_opt::<RcvBuf>(size)
    }

    pub fn send_buf(self, size: u32) -> Result<Self> {
        self.set_opt::<SndBuf>(size)
    }

    pub fn get_reuse_addr(&self) -> Result<bool> {
        self.get_opt::<ReuseAddr>()
    }

    pub fn get_reuse_port(&self) -> Result<bool> {
        self.get_opt::<ReusePort>()
    }

    pub fn get_dont_route(&self) -> Result<bool> {
        self.get_opt::<DontRoute>()
    }

    pub fn get_linger(&self) -> Result<Option<u32>> {
        let linger = self.get_opt::<Linger>()?;

        if linger.enabled == 0 {
            Ok(None)
        } else {
            Ok(Some(linger.seconds))
        }
    }

    pub fn get_recv_buf(&self) -> Result<u32> {
        self.get_opt::<RcvBuf>()
    }

    pub fn get_send_buf(&self) -> Result<u32> {
        self.get_opt::<SndBuf>()
    }
}

impl<Addr: socket_addr> builder<Addr, Stream> {
    pub fn keep_alive(self, flag: bool) -> Result<Self> {
        self.set_opt::<KeepAlive>(flag)
    }

    pub fn get_keep_alive(&self) -> Result<bool> {
        self.get_opt::<KeepAlive>()
    }
}

impl<Addr: socket_addr> builder<Addr, Datagram> {
    pub fn boardcast(self, flag: bool) -> Result<Self> {
        self.set_opt::<Broadcast>(flag)
    }

    pub fn get_broadcast(&self) -> Result<bool> {
        self.get_opt::<Broadcast>()
    }
}
