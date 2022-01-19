mod close;
mod convert;
mod dup;
mod mmap;
mod pipe;
mod reader;
mod seek;
mod writer;

pub use close::Close;
pub use mmap::MMap;
pub use pipe::Pipe;
pub use reader::Reader;
pub use seek::Seek;
pub use writer::Writer;

#[derive(Debug, PartialEq)]
pub struct fd {
    pub(crate) fd: i32,
}

#[derive(Debug, PartialEq)]
pub struct rfd(pub(crate) fd);
#[derive(Debug, PartialEq)]
pub struct wfd(pub(crate) fd);

impl fd {
    pub fn stdin() -> rfd {
        rfd(fd { fd: 0 })
    }

    pub fn stdout() -> wfd {
        wfd(fd { fd: 1 })
    }

    pub fn stderr() -> wfd {
        wfd(fd { fd: 2 })
    }

    pub(crate) fn new(fd: i32) -> fd {
        fd { fd }
    }
}
