mod open;
mod pipe;
mod reader;
mod writer;
mod close;
mod dup;

pub use reader::Reader;
pub use writer::Writer;
pub use close::Close;
pub use pipe::Pipe;

pub use open::*;

#[link(name = "c")]
extern "C" {
    fn open(path: *const u8, flags: u32) -> i32;
}

pub struct fd {
    fd: i32,
}

pub struct rfd(fd);
pub struct wfd(fd);

impl fd {
    pub const STDIN: rfd = rfd(fd { fd: 0 });
    pub const STDOUT: wfd = wfd(fd { fd: 1 });
    pub const STDERR: wfd = wfd(fd { fd: 2 });
}

