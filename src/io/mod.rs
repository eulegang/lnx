mod pipe;
mod reader;
mod writer;
mod close;
mod dup;
mod convert;

pub use reader::Reader;
pub use writer::Writer;
pub use close::Close;
pub use pipe::Pipe;

#[derive(Debug, PartialEq)]
pub struct fd {
    fd: i32,
}

#[derive(Debug, PartialEq)]
pub struct rfd(fd);
#[derive(Debug, PartialEq)]
pub struct wfd(fd);

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

