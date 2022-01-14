use crate::syscall::{accept, bind, connect, listen, socket};
use crate::{
    io::{fd, Reader, Writer},
    Result, ToErrno,
};
use core::marker::PhantomData;

mod addr;

pub use addr::*;

#[repr(u32)]
pub enum SocketType {
    Stream = 1,
    Datagram = 2,
}

pub struct socket {
    fd: fd,
}

impl socket {
    pub fn connect<Addr>(addr: Addr, ty: SocketType) -> Result<socket>
    where
        Addr: addr::socket_addr,
    {
        let sock = socket(Addr::FAMILY as u32, ty as u32, 0);
        let fd = sock.to_errno()? as i32;
        let fd = fd { fd };

        connect(fd.fd, (&addr as *const Addr).cast(), Addr::SIZE).to_errno()?;

        Ok(socket { fd })
    }
}

pub struct listen<Addr: addr::socket_addr> {
    fd: fd,
    _address: PhantomData<Addr>,
}

impl<Addr: addr::socket_addr> listen<Addr> {
    pub fn listening(addr: Addr, ty: SocketType) -> Result<listen<Addr>>
    where
        Addr: addr::socket_addr,
    {
        let sock = socket(Addr::FAMILY as u32, ty as u32, 0);
        let fd = sock.to_errno()? as i32;
        let fd = fd { fd };

        bind(fd.fd, (&addr as *const Addr).cast(), Addr::SIZE).to_errno()?;
        listen(fd.fd, 10).to_errno()?;

        let _address = PhantomData;
        Ok(listen { fd, _address })
    }

    pub fn accept(&self, addr: Option<&mut Addr>) -> Result<socket> {
        let addr = match addr {
            Some(addr) => (addr as *mut Addr).cast(),
            None => core::ptr::null_mut(),
        };

        let fd = accept(self.fd.fd, addr, core::ptr::null_mut()).to_errno()? as i32;

        let fd = fd { fd };

        Ok(socket { fd })
    }
}

impl Reader for socket {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.fd.read(buf)
    }
}

impl Writer for socket {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.fd.write(buf)
    }
}

#[test]
fn test_socket() {
    let addr: in4_addr = [0, 0, 0, 0].into();
    let listen_socket: listen<IPv4> =
        listen::listening((addr, 12345).into(), SocketType::Stream).unwrap();

    std::thread::spawn(|| {
        let addr: in4_addr = [127, 0, 0, 1].into();

        let mut write = socket::connect::<IPv4>((addr, 12345).into(), SocketType::Stream).unwrap();

        write.write(b"hello world!").unwrap();
    });

    let mut read = listen_socket.accept(None).unwrap();

    let mut buffer = [0u8; 128];

    let n = read.read(&mut buffer).unwrap();

    assert_eq!(&buffer[0..n], b"hello world!");
}
