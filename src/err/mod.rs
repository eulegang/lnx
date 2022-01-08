use core::num::NonZeroU32;

#[derive(Debug, PartialEq)]
pub struct Errno {
    err: NonZeroU32,
}

impl Errno {
    pub fn new(result: i32) -> Result<u32, Errno> {
        if result < 0 {
            let err = unsafe { NonZeroU32::new_unchecked((-result) as u32) };

            Err(Errno { err })
        } else {
            Ok(result as u32)
        }
    }
}

