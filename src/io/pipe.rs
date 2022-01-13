use crate::{ToErrno, syscall::pipe2, konst::*, Result};
use super::{fd, wfd, rfd};


impl fd {
    pub fn pipe() -> Result<(rfd, wfd)> {
        let mut fds = [0i32; 2];
        let status = pipe2(&mut fds, 0);

        status.to_errno()?;

        Ok((rfd(fd { fd: fds[0] }), wfd(fd { fd: fds[1] })))
    }
}

pub struct Pipe {
    flags: u32
}

impl Pipe {
    pub const CLOEXEC: Pipe = Pipe { flags: O_CLOEXEC };
    pub const NONBLOCK: Pipe = Pipe { flags: O_NONBLOCK };

    pub fn open(self) -> Result<(rfd, wfd)> {
        let mut fds = [0i32; 2];
        pipe2(&mut fds, self.flags).to_errno()?;

        Ok((rfd(fd { fd: fds[0] }), wfd(fd { fd: fds[1] })))
    }
}

impl core::ops::BitOr for Pipe {
    type Output = Pipe;
    fn bitor(self, open: Pipe) -> Pipe {
        Pipe {
            flags: self.flags | open.flags,
        }
    }
}

impl core::ops::BitAnd for Pipe {
    type Output = Pipe;
    fn bitand(self, open: Pipe) -> Pipe {
        Pipe {
            flags: self.flags & open.flags,
        }
    }
}

impl core::ops::Not for Pipe {
    type Output = Pipe;

    fn not(self) -> Pipe {
        Pipe { flags: !self.flags }
    }
}

#[test]
fn test_pipe() {
    use crate::prelude::*;

    let (mut r, mut w) = fd::pipe().unwrap();

    w.write(b"hello world").unwrap();

    let mut bytes = [0u8; 1024];

    let len = r.read(&mut bytes).unwrap();

    assert_eq!(len, 11);
    assert_eq!(&bytes[0..len], b"hello world");

    r.close().unwrap();
    w.close().unwrap();
}
