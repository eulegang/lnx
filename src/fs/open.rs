use crate::{
    io::{fd, rfd, wfd},
    konst::*,
    syscall::open,
    Result, ToErrno,
};

use lnx_flags::Flags;

#[derive(Flags, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rd {
    flags: u32,
}

impl Default for Rd {
    fn default() -> Rd {
        Rd { flags: O_RDONLY }
    }
}

impl Rd {
    pub const ASYNC: Rd = Rd { flags: O_ASYNC };
    pub const CLOEXEC: Rd = Rd { flags: O_CLOEXEC };
    pub const NONBLOCK: Rd = Rd { flags: O_NONBLOCK };

    pub fn open(&self, path: &[u8]) -> Result<rfd> {
        let fd = open(path.as_ptr(), O_RDONLY | self.flags, 0o777).to_errno()? as i32;

        Ok(fd::new(fd).into())
    }
}

#[derive(Flags, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Wr {
    flags: u32,
}

impl Default for Wr {
    fn default() -> Wr {
        Wr { flags: O_WRONLY }
    }
}

impl Wr {
    pub const APPEND: Open = Open { flags: O_APPEND };
    pub const TRUNC: Open = Open { flags: O_TRUNC };
    pub const ASYNC: Open = Open { flags: O_ASYNC };
    pub const CLOEXEC: Open = Open { flags: O_CLOEXEC };
    pub const DSYNC: Open = Open { flags: O_DSYNC };
    pub const EXCL: Open = Open { flags: O_EXCL };
    pub const NOFOLLOW: Open = Open { flags: O_NOFOLLOW };
    pub const NONBLOCK: Open = Open { flags: O_NONBLOCK };
    pub const SYNC: Open = Open { flags: O_SYNC };

    pub fn open(&self, path: &[u8]) -> Result<wfd> {
        self.open_perms(path, 0o777)
    }

    pub fn open_perms(&self, path: &[u8], perms: u32) -> Result<wfd> {
        let fd = open(path.as_ptr(), self.flags, perms).to_errno()? as i32;

        Ok(fd::new(fd).into())
    }
}

#[derive(Flags, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Open {
    flags: u32,
}

impl Default for Open {
    fn default() -> Open {
        Open { flags: O_RDWR }
    }
}

impl Open {
    pub const APPEND: Open = Open { flags: O_APPEND };
    pub const TRUNC: Open = Open { flags: O_TRUNC };
    pub const ASYNC: Open = Open { flags: O_ASYNC };
    pub const CLOEXEC: Open = Open { flags: O_CLOEXEC };
    pub const DSYNC: Open = Open { flags: O_DSYNC };
    pub const EXCL: Open = Open { flags: O_EXCL };
    pub const NOFOLLOW: Open = Open { flags: O_NOFOLLOW };
    pub const NONBLOCK: Open = Open { flags: O_NONBLOCK };
    pub const SYNC: Open = Open { flags: O_SYNC };

    pub fn open(&self, path: &[u8]) -> Result<fd> {
        self.open_perms(path, 0o777)
    }

    pub fn open_perms(&self, path: &[u8], perms: u32) -> Result<fd> {
        let fd = open(path.as_ptr(), self.flags, perms).to_errno()? as i32;

        Ok(fd::new(fd))
    }
}

#[test]
fn read_manifest() {
    use crate::io::Reader;

    let mut buf = [0u8; 12];
    let mut fd = Rd::default().open(b"Cargo.toml\0").unwrap();

    assert_eq!(fd.read(&mut buf), Ok(12));
    assert_eq!(&buf, b"[package]\nna");
}

#[test]
fn missing() {
    use crate::Errno;

    assert_eq!(Rd::default().open(b"/foobar\0"), Err(Errno::ENOENT));
}

#[test]
fn write_null() {
    use crate::io::{Close, Writer};

    let mut wr = Wr::default().open(b"/dev/null\0").unwrap();

    assert_eq!(wr.write(b"hello world").unwrap(), 11);
    assert_eq!(wr.close(), Ok(()));
}
