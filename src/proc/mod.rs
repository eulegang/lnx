use crate::{
    Result,
    ToErrno,
    syscall::{
        exit as sys_exit,
        fork as sys_fork,
        vfork as sys_vfork,
        getpid,
    },
};
use core::num::NonZeroU32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct pid {
    pid: NonZeroU32,
}

pub enum Fork {
    Parent(pid),
    Child
}

pub struct Proc;

impl Proc {
    pub fn exit(code: i32) -> ! {
        sys_exit(code);
        unreachable!()
    }

    pub fn id() -> pid {
        let pid = getpid() as u32;
        let pid = unsafe { NonZeroU32::new_unchecked(pid) };
        pid { pid }
    }

    pub fn fork() -> Result<Fork> {
        let pid = sys_fork().to_errno()?;

        match pid {
            0 => Ok(Fork::Child),
            pid => {
                let pid = unsafe { NonZeroU32::new_unchecked(pid) };
                Ok(Fork::Parent(pid { pid }))
            }
        }
    }

    pub fn vfork() -> Result<Fork> {
        let pid = sys_vfork().to_errno()?;

        match pid {
            0 => Ok(Fork::Child),
            pid => {
                let pid = unsafe { NonZeroU32::new_unchecked(pid) };
                Ok(Fork::Parent(pid { pid }))
            }
        }
    }
}
