use crate::syscall::{accept, listen, socket};
use crate::{
    io::{fd, Reader, Writer},
    Result, ToErrno,
};
use core::marker::PhantomData;

mod addr;
mod create;
mod opt;

pub use addr::*;
pub use create::*;

pub struct socket {
    fd: fd,
}

pub struct listen<Addr: addr::socket_addr> {
    fd: fd,
    _address: PhantomData<Addr>,
}

impl<Addr: addr::socket_addr> listen<Addr> {
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
    let listen_socket: listen<IPv4> = builder::<IPv4, { SocketType::Stream }>::new()
        .unwrap()
        .reuse_addr(true)
        .unwrap()
        .listen((addr, 12345).into())
        .unwrap();

    let listen_socket = listen_socket;

    let addr: in4_addr = [127, 0, 0, 1].into();

    let mut write = builder::<IPv4, { SocketType::Stream }>::new()
        .unwrap()
        .connect((addr, 12345).into())
        .unwrap();

    write.write(b"hello world!").unwrap();

    let mut read = listen_socket.accept(None).unwrap();

    let mut buffer = [0u8; 128];

    let n = read.read(&mut buffer).unwrap();

    assert_eq!(&buffer[0..n], b"hello world!");
}
