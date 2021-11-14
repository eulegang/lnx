use crate::StrExt;

#[link(name = "c")]
extern {
    static mut errno: i32;

    fn strerror(errno: i32) -> *const u8;
}

pub struct SysErr {
    err: i32 
}

impl SysErr {
    pub fn take() -> SysErr {
        let err = unsafe { errno };
        SysErr { err }
    }

    pub fn check_syscall(res: i32) -> Option<SysErr> {
        if res == -1 {
            let err = unsafe { errno };
            Some(SysErr { err })
        } else {
            None
        }
    }

    pub fn as_str(&self) -> &str {
        let msg = unsafe { strerror(self.err) };
        str::from_cstr(msg)
    }
}

pub(crate) trait SysErrOptExt {
    fn check<T>(self, provide: impl Fn() -> T) -> Result<T, SysErr>;
}

impl SysErrOptExt for Option<SysErr> {
    fn check<T>(self, provide: impl Fn() -> T) -> Result<T, SysErr> {
        match self {
            Some(s) => Err(s),
            None => Ok(provide()),
        }
    }
}

