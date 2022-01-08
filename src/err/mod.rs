#[derive(Debug, PartialEq)]
pub struct SysErr {
    err: i32,
}

impl SysErr {
    pub fn take() -> SysErr {
        let err = unsafe { todo!() };
        SysErr { err }
    }

    pub fn check_syscall(res: i32) -> Option<SysErr> {
        if res == -1 {
            Some(SysErr::take())
        } else {
            None
        }
    }
}

