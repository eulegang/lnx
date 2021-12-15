use super::{fd, rfd};
use crate::{SysErr, CStr};
use crate::konst::*;
use crate::syscall::open;

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

    pub fn open(&self, path: &CStr) -> Result<rfd, SysErr> {
        let fd = open(path.as_ptr(), O_RDONLY | self.flags, 0o777);

        if fd == -1 {
            Err(SysErr::take())
        } else {
            Ok(rfd(fd { fd }))
        }
    }
}

pub struct Open {
    flags: u32,
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

    pub fn open(&self, path: &CStr) -> Result<fd, SysErr> {
        self.open_perms(path, 0o777)
    }

    pub fn open_perms(&self, path: &CStr, perms: u32) -> Result<fd, SysErr> {
        let fd = open(path.as_ptr(), self.flags, perms);

        if fd == -1 {
            Err(SysErr::take())
        } else {
            Ok(fd { fd })
        }
    }
}

flag_impl!(Open);
flag_impl!(Rd);


#[test]
fn read_manifest() {
    use crate::prelude::*;

    let mut buf = [0u8; 12];
    let mut fd = Rd::default().open(&CStr::new("Cargo.toml").unwrap()).unwrap();

    assert_eq!(fd.read(&mut buf), Ok(12));
    assert_eq!(&buf, b"[package]\nna");
}

