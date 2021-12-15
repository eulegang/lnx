use super::fd;
use crate::{SysErr, CStr};
use crate::konst::*;
use crate::syscall::open;

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

impl core::ops::BitOr for Open {
    type Output = Open;
    fn bitor(self, open: Open) -> Open {
        Open {
            flags: self.flags | open.flags,
        }
    }
}

impl core::ops::BitAnd for Open {
    type Output = Open;
    fn bitand(self, open: Open) -> Open {
        Open {
            flags: self.flags & open.flags,
        }
    }
}

impl core::ops::Not for Open {
    type Output = Open;

    fn not(self) -> Open {
        Open { flags: !self.flags }
    }
}
