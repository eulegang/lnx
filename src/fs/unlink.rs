use crate::{
    Result,
    SysErr,
    syscall::unlink,
};

pub fn rm(path: &[u8]) -> Result<()> {
    let err = unlink(path.as_ptr());

    if err == -1 {
        Err(SysErr::take())
    } else {
        Ok(())
    }
}
