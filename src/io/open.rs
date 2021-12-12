use super::{fd, open};
use crate::{SysErr, CStr};

pub struct Open {
    flags: u32,
}

impl Open {
    pub const APPEND: Open = Open { flags: 0x400 };
    pub const TRUNC: Open = Open { flags: 0x200 };
    pub const ASYNC: Open = Open { flags: 0x2000 };
    pub const CLOEXEC: Open = Open { flags: 0x80000 };
    pub const DSYNC: Open = Open { flags: 0x1000 };
    pub const EXCL: Open = Open { flags: 0x80 };
    pub const NOFOLLOW: Open = Open { flags: 0x20000 };
    pub const NONBLOCK: Open = Open { flags: 0x800 };
    pub const SYNC: Open = Open { flags: 0x101000 };

    pub fn open(&self, path: &CStr) -> Result<fd, SysErr> {
        let fd = unsafe { open(path.as_ptr(), self.flags) };

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
