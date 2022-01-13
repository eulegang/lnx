use crate::{
    Result,
    ToErrno,
    syscall::unlink,
};

pub fn rm(path: &[u8]) -> Result<()> {
    unlink(path.as_ptr()).to_errno()?;
    Ok(())
}
