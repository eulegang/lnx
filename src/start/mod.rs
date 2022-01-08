use core::marker::PhantomData;

#[repr(transparent)]
pub struct Start {
    addr: *const usize,
}

impl Start  {
    pub fn args(&self) -> Args {
        let addr = self.addr;
        let offset = 0;
        let len = unsafe { self.addr.read() };
        let _life = PhantomData;

        Args { _life, addr, offset, len }
    }
}

pub struct Args<'a> {
    _life: PhantomData<&'a ()>,
    addr: *const usize,
    offset: usize,
    len: usize,
}

impl Args<'_> {
    pub fn len(&self) -> usize{
        self.len
    }
}

impl<'a> Iterator for Args<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<&'a [u8]> {
        self.offset += 1;

        if self.offset > self.len {
            None
        } else {
            let ptr = unsafe { self.addr.add(self.offset).read() as *const u8 };
            Some(slice_from_cstr(ptr))
        }
    }
}

fn slice_from_cstr(ptr: *const u8) -> &'static [u8] {
    unsafe {
        let mut it = ptr;
        let mut len = 0;

        while it.read() != 0 {
            len += 1;
            it = it.add(1);
        }

        core::slice::from_raw_parts(ptr, len)
    }
}
