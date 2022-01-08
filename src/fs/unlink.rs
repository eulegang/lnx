use crate::{
    Result,
    Errno,
    syscall::unlink,
};

pub fn rm(path: &[u8]) -> Result<()> {
    Errno::new(unlink(path.as_ptr()))?;
    Ok(())
}
