use crate::{
    Result,
    SysErr,
    CStr,
    syscall::unlink,
};

pub fn rm(path: &CStr) -> Result<()> {
    let err = unlink(path.as_ptr());

    if err == -1 {
        Err(SysErr::take())
    } else {
        Ok(())
    }
}
