use crate::StrExt;

#[link(name = "c")]
extern "C" {
    //static mut errno: i32;
    
    #[link_name = "__errno_location"]
    fn errno_location() -> *mut i32;

    fn strerror(errno: i32) -> *const u8;
}

#[derive(Debug, PartialEq)]
pub struct SysErr {
    err: i32,
}

impl SysErr {
    pub fn take() -> SysErr {
        let err = unsafe { *errno_location() };
        SysErr { err }
    }

    pub fn check_syscall(res: i32) -> Option<SysErr> {
        if res == -1 {
            Some(SysErr::take())
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

