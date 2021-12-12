use super::StrExt;
use crate::prelude::*;

#[test]
fn test_cstr() {
    let s = b"foobar\0".as_ptr();

    assert_eq!(str::from_cstr(s), "foobar");
}

#[test]
fn test_new() {
    let s = CStr::new("hello, world").unwrap();

    assert_eq!(s.as_ref(), "hello, world");

    let slice = unsafe { core::slice::from_raw_parts(s.as_ptr(), s.len()+1) };

    assert_eq!(slice, b"hello, world\0");
}

#[test]
fn test_clone() {
    let s = CStr::new("hello, world").unwrap();
    let a = s.clone();

    assert!(a.as_ptr() != s.as_ptr());
}
