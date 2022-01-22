use core::marker::PhantomData;

use crate::err::ToErrno;
use crate::io::fd;
use crate::socket::addr::socket_addr;
use crate::syscall::{bind, connect, socket as sys_socket};
use crate::Result;

use super::{listen, socket};

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SocketType {
    Stream = 1,
    Datagram = 2,
}

impl SocketType {
    pub fn builder<Addr: socket_addr>(self) -> Result<builder<Addr>> {
        let sock = sys_socket(Addr::FAMILY as u32, self as u32, 0);
        let fd = sock.to_errno()? as i32;
        let fd = fd { fd };

        let _addr = PhantomData;

        Ok(builder { fd, _addr })
    }
}

pub struct builder<Addr: socket_addr> {
    pub(crate) fd: fd,
    _addr: PhantomData<Addr>,
}

impl<Addr: socket_addr> builder<Addr> {
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
}
