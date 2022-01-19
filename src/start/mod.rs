use core::marker::PhantomData;

#[repr(transparent)]
pub struct Start {
    addr: *const usize,
}

impl Start {
    pub fn args(&self) -> Args {
        let addr = self.addr;
        let offset = 0;
        let len = unsafe { self.addr.read() };
        let _life = PhantomData;

        Args {
            _life,
            addr,
            offset,
            len,
        }
    }
}

pub struct Args<'a> {
    _life: PhantomData<&'a ()>,
    addr: *const usize,
    offset: usize,
    len: usize,
}

impl<'a> Args<'a> {
    pub fn get(&self, idx: usize) -> Option<&'a [u8]> {
        if idx >= self.len {
            return None;
        }

        Some(self.get_unchecked(idx))
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn get_unchecked(&self, idx: usize) -> &'a [u8] {
        let ptr = unsafe { self.addr.add(idx + 1).read() as *const u8 };
        slice_from_cstr(ptr)
    }
}

impl<'a> Iterator for Args<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<&'a [u8]> {
        let cur = self.offset;
        self.offset += 1;

        if self.offset > self.len {
            None
        } else {
            Some(self.get_unchecked(cur))
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
