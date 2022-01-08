use crate::{
    Errno, 
    Result,
    syscall::open,
    io::{fd, rfd, wfd},
    konst::*,
};

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

    fn new(flags: u32) -> Rd {
        Rd { flags }
    }

    pub fn open(&self, path: &[u8]) -> Result<rfd> {
        let fd = Errno::new(open(path.as_ptr(), O_RDONLY | self.flags, 0o777))? as i32;

        Ok(fd::new(fd).into())
    }
}

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

    pub fn new(flags: u32) -> Wr {
        Wr { flags }
    }

    pub fn open(&self, path: &[u8]) -> Result<wfd> {
        self.open_perms(path, 0o777)
    }

    pub fn open_perms(&self, path: &[u8], perms: u32) -> Result<wfd> {
        let fd = Errno::new(open(path.as_ptr(), self.flags, perms))? as i32;

        Ok(fd::new(fd).into())
    }
}

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

    fn new(flags: u32) -> Open {
        Open { flags }
    }

    pub fn open(&self, path: &[u8]) -> Result<fd> {
        self.open_perms(path, 0o777)
    }

    pub fn open_perms(&self, path: &[u8], perms: u32) -> Result<fd> {
        let fd = Errno::new(open(path.as_ptr(), self.flags, perms))? as i32;

        Ok(fd::new(fd))
    }
}

flag_impl!(Open);
flag_impl!(Rd);
flag_impl!(Wr);

#[test]
fn read_manifest() {
    use crate::prelude::*;

    let mut buf = [0u8; 12];
    let mut fd = Rd::default().open(b"Cargo.toml\0").unwrap();

    assert_eq!(fd.read(&mut buf), Ok(12));
    assert_eq!(&buf, b"[package]\nna");
}

#[test]
fn missing() {
    assert_eq!(Rd::default().open(b"/foobar\0"), Err(Errno::ENOENT));
}

#[test]
fn write_null() {
    use crate::prelude::*;
    let mut wr = Wr::default().open(b"/dev/null\0").unwrap();

    assert_eq!(wr.write(b"hello world").unwrap(), 11);
    assert_eq!(wr.close(), Ok(()));
}
