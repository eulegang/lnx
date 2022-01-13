mod pipe;
mod reader;
mod writer;
mod close;
mod seek;
mod dup;
mod convert;

pub use reader::Reader;
pub use writer::Writer;
pub use close::Close;
pub use pipe::Pipe;
pub use seek::Seek;

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

