use super::{fd, rfd, wfd};
use crate::{syscall::lseek, Result, ToErrno};

const SEEK_SET: u32 = 0;
const SEEK_CUR: u32 = 1;
const SEEK_END: u32 = 2;

pub enum Destination {
    Set(usize),
    Cur(usize),
    End(usize),
}

pub trait Seek {
    fn seek(&mut self, to: Destination) -> Result<usize>;

    fn size(&mut self) -> Result<usize> {
        self.seek(Destination::End(0))
    }

    fn reset(&mut self) -> Result<()> {
        self.seek(Destination::Set(0))?;
        Ok(())
    }

    fn tell(&mut self) -> Result<usize> {
        self.seek(Destination::Cur(0))
    }
}

impl Seek for fd {
    fn seek(&mut self, to: Destination) -> Result<usize> {
        let res = match to {
            Destination::Set(s) => lseek(self.fd, s, SEEK_SET),
            Destination::Cur(s) => lseek(self.fd, s, SEEK_CUR),
            Destination::End(s) => lseek(self.fd, s, SEEK_END),
        };

        res.to_errno()
    }
}

impl Seek for rfd {
    fn seek(&mut self, to: Destination) -> Result<usize> {
        self.0.seek(to)
    }
}

impl Seek for wfd {
    fn seek(&mut self, to: Destination) -> Result<usize> {
        self.0.seek(to)
    }
}

#[test]
fn seek_reset() {
    use crate::fs::Rd;
    use crate::io::Reader;

    let mut buf = [0u8; 12];
    let mut fd = Rd::default().open(b"Cargo.toml\0").unwrap();

    assert_eq!(fd.read(&mut buf), Ok(12));
    assert_eq!(&buf, b"[package]\nna");

    fd.reset().unwrap();

    assert_eq!(fd.read(&mut buf), Ok(12));
    assert_eq!(&buf, b"[package]\nna");

    assert_eq!(fd.read(&mut buf), Ok(12));
    assert_eq!(&buf, b"me = \"lnx\"\nv");
}
