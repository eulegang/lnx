use super::{fd, rfd, Seek};
use crate::mmap::{mmap, mregion, Protect};
use crate::Result;

pub trait MMap {
    fn mmap(self) -> Result<mregion>;
}

impl MMap for fd {
    fn mmap(mut self) -> Result<mregion> {
        mmap::default()
            .len(self.size()?)
            .protect(Protect::READ)
            .file(self)
            .map()
    }
}

impl MMap for rfd {
    fn mmap(self) -> Result<mregion> {
        self.0.mmap()
    }
}

#[test]
fn test_fd_mmap() {
    use crate::fs::Rd;

    let fd = Rd::default().open(b"Cargo.toml\0").unwrap();

    let region = fd.mmap().unwrap();

    assert_eq!(&region[0..12], b"[package]\nna");

    assert!(region.len() != 12);
}
