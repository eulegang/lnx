use crate::io::{fd, Close};
use crate::syscall::{mmap, munmap};
use crate::{Result, ToErrno};

#[derive(Debug, PartialEq, Eq)]
pub struct Protect {
    flags: usize,
}

impl Protect {
    fn new(flags: usize) -> Protect {
        Protect { flags }
    }

    pub const NONE: Protect = Protect { flags: 0 };
    pub const READ: Protect = Protect { flags: 1 };
    pub const WRITE: Protect = Protect { flags: 2 };
    pub const EXEC: Protect = Protect { flags: 4 };
}

impl Default for Protect {
    fn default() -> Protect {
        Protect::NONE
    }
}

pub struct Map {
    flags: usize,
}

impl Map {
    fn new(flags: usize) -> Map {
        Map { flags }
    }

    pub const SHARED: Map = Map { flags: 1 };
    pub const PRIVATE: Map = Map { flags: 2 };
    pub const ANONYMOUS: Map = Map { flags: 32 };
}

pub struct mmap {
    protect: Protect,
    len: usize,
    offset: usize,
    share: bool,
    file: Option<fd>,
}

flag_impl!(Map);
flag_impl!(Protect);

impl mmap {
    pub fn len(mut self, len: usize) -> Self {
        self.len = len;
        self
    }

    pub fn protect(mut self, protect: Protect) -> Self {
        self.protect = protect;
        self
    }

    pub fn share(mut self) -> Self {
        self.share = true;
        self
    }

    pub fn file(mut self, fd: fd) -> Self {
        self.file = Some(fd);
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    pub fn map(self) -> Result<mregion> {
        let fd: usize;
        if let Some(s) = &self.file {
            fd = s.fd as usize
        } else {
            fd = usize::MAX;
        }
        let len = self.len;

        let mut flags = 0;

        if fd == usize::MAX {
            flags |= Map::ANONYMOUS.flags;
        }

        match self.share {
            false => flags |= Map::PRIVATE.flags,
            true => flags |= Map::SHARED.flags,
        };

        let res = mmap(
            core::ptr::null_mut(),
            len,
            self.protect.flags,
            flags,
            fd,
            self.offset,
        );

        let addr = res.to_errno()? as *mut u8;

        if let Some(file) = self.file {
            file.close()?;
        }

        Ok(mregion { addr, len })
    }
}

impl Default for mmap {
    fn default() -> mmap {
        let protect = Protect::NONE;
        let len = 0;
        let offset = 0;
        let share = false;
        let file = None;

        mmap {
            protect,
            len,
            offset,
            share,
            file,
        }
    }
}

pub struct mregion {
    addr: *mut u8,
    len: usize,
}

impl mregion {
    pub fn close(self) -> Result<()> {
        munmap(self.addr, self.len).to_errno()?;

        Ok(())
    }
}

impl Drop for mregion {
    fn drop(&mut self) {
        let _ = munmap(self.addr, self.len);
    }
}

impl core::ops::Deref for mregion {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.as_ref()
    }
}

impl AsRef<[u8]> for mregion {
    fn as_ref(&self) -> &[u8] {
        let addr: *const u8 = self.addr.cast();

        unsafe { core::slice::from_raw_parts(addr, self.len) }
    }
}

impl AsMut<[u8]> for mregion {
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.addr, self.len) }
    }
}

#[test]
fn map_anon() {
    let mut region = mmap::default()
        .protect(Protect::WRITE | Protect::READ)
        .len(128)
        .map()
        .unwrap();

    region.as_mut()[0..11].copy_from_slice(b"hello world");

    assert_eq!(&region[0..11], b"hello world");
    assert_eq!(region.len(), 128);
}

#[test]
fn map_manifest() {
    use crate::fs::Rd;

    let fd = Rd::default().open(b"Cargo.toml\0").unwrap();

    let region = mmap::default()
        .protect(Protect::READ)
        .len(128)
        .file(fd.0)
        .map()
        .unwrap();

    assert_eq!(&region[0..12], b"[package]\nna");
}
