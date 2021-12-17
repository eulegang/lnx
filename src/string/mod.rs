#[cfg(test)]
mod test;

use crate::mem::{Alloc, Sys};

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

        unsafe { core::str::from_utf8_unchecked(slice) }
    }
}

pub struct CStr<A: Alloc = Sys> {
    ptr: *const u8,
    len: usize,
    alloc: A,
}

impl CStr {
    pub fn new(s: &str) -> Option<CStr> {
        let len = s.len();
        let mem = unsafe { Sys.malloc(s.len()+1) };
        if mem.is_null() {
            return None;
        }

        unsafe { s.as_ptr().copy_to(mem, s.len()); }
        unsafe { mem.add(s.len()).write(0); }

        Some(CStr { ptr: mem, len, alloc: Sys })
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.ptr
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl AsRef<str> for CStr {
    fn as_ref(&self) -> &str {
        let slice = unsafe { core::slice::from_raw_parts(self.ptr, self.len) };
        unsafe { core::str::from_utf8_unchecked(slice) }
    }
}

impl core::ops::Deref for CStr {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_ref()
    }
}

impl<A: Alloc> Drop for CStr<A> {
    fn drop(&mut self) {
        unsafe { self.alloc.free(self.ptr as *mut u8); }
    }
}

impl Clone for CStr {
    fn clone(&self) -> CStr {
        let len = self.len;
        let alloc = self.alloc;
        let ptr = unsafe { self.alloc.malloc(len + 1) };
        if ptr.is_null() {
            panic!("out of memory");
        }

        unsafe { self.ptr.copy_to(ptr, len); }
        unsafe { ptr.add(len).write(0); }

        CStr { ptr, len, alloc }
    }
}
