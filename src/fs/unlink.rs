use crate::{syscall::unlink, Result, ToErrno};

pub fn rm(path: &[u8]) -> Result<()> {
    unlink(path.as_ptr()).to_errno()?;
    Ok(())
}
