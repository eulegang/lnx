use core::mem::size_of;
use core::ptr::copy;

#[link(name = "c")]
extern "C" {
    /// # Safety
    ///
    /// Do not use with 0
    fn malloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
    fn realloc(ptr: *mut u8, size: usize) -> *mut u8;
}

pub trait Alloc: Clone {
    unsafe fn malloc(&self, size: usize) -> *mut u8;
    unsafe fn realloc(&self, ptr: *mut u8, size: usize) -> *mut u8;
    unsafe fn free(&self, ptr: *mut u8);
}

#[derive(Clone, Copy)]
pub struct Sys;

impl Alloc for Sys {
    unsafe fn malloc(&self, size: usize) -> *mut u8 {
        malloc(size)
    }

    unsafe fn realloc(&self, ptr: *mut u8, size: usize) -> *mut u8 {
        realloc(ptr, size)
    }

    unsafe fn free(&self, ptr: *mut u8) {
        free(ptr);
    }
}

pub struct heaped<T, A: Alloc = Sys> {
    ptr: *mut T,
    alloc: A,
}

impl<T, A> heaped<T, A>
where
    A: Alloc,
{
    pub fn new(item: T, alloc: A) -> Option<heaped<T, A>> {
        let ptr = unsafe { alloc.malloc(size_of::<T>()) };
        let ptr = ptr.cast::<T>();

        if ptr.is_null() {
            None
        } else {
            unsafe { copy(&item, ptr, 1) };

            Some(heaped { ptr, alloc })
        }
    }
}

impl<T, A> AsRef<T> for heaped<T, A>
where
    A: Alloc,
{
    fn as_ref(&self) -> &T {
        unsafe { &*self.ptr as &T }
    }
}

impl<T, A> Drop for heaped<T, A>
where
    A: Alloc,
{
    fn drop(&mut self) {
        unsafe { self.alloc.free(self.ptr.cast()) }
    }
}
