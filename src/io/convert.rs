use super::{fd, rfd, wfd};

impl From<fd> for rfd {
    fn from(fd: fd) -> rfd {
        rfd(fd)
    }
}

impl From<fd> for wfd {
    fn from(fd: fd) -> wfd {
        wfd(fd)
    }
}
