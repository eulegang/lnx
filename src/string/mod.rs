#[link(name = "c")]
extern "C" {
    fn strlen(cstr: *const u8) -> usize;
}

pub(crate) trait StrExt {
    fn from_cstr<'a>(cstr: *const u8) -> &'a str;
}

impl StrExt for str {
    fn from_cstr<'a>(cstr: *const u8) -> &'a str {
        let len = unsafe { strlen(cstr) };
        let slice = unsafe { core::slice::from_raw_parts(cstr, len) };

        unsafe { core::intrinsics::transmute(slice) }
    }
}
