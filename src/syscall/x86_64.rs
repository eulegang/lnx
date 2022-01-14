use core::arch::asm;

pub(crate) fn pipe2(fds: &mut [i32; 2], flags: u32) -> i32 {
    let ret: i32;
    unsafe {
        asm!(
            "syscall",

            in("rax") 293,
            in("rdi") fds,
            in("esi") flags,
            lateout("eax") ret,
        );
    }

    ret
}

pub(crate) fn close(fd: i32) -> i32 {
    let ret: i32;
    unsafe {
        asm!(
            "syscall",

            in("rax") 3,
            in("edi") fd,
            lateout("eax") ret,
        );
    }

    ret
}

pub(crate) fn read(fd: i32, bytes: &mut [u8]) -> i32 {
    let ret: i32;
    let len = bytes.len();
    let ptr = bytes.as_ptr();

    unsafe {
        asm!(
            "syscall",

            in("rax") 0,
            in("edi") fd,
            in("rsi") ptr,
            in("rdx") len,

            lateout("eax") ret,
        );
    }

    ret
}

pub(crate) fn open(path: *const u8, flags: u32, perms: u32) -> i32 {
    let ret: i32;

    unsafe {
        asm!(
            "syscall",

            in("rax") 2,
            in("rdi") path,
            in("rsi") flags,
            in("rdx") perms,

            lateout("eax") ret,
        );
    }

    ret
}

pub(crate) fn write(fd: i32, bytes: &[u8]) -> i32 {
    let ret: i32;
    let len = bytes.len();
    let ptr = bytes.as_ptr();

    unsafe {
        asm!(
            "syscall",

            in("rax") 1,
            in("edi") fd,
            in("rsi") ptr,
            in("rdx") len,

            lateout("eax") ret,
        );
    }

    ret
}

pub(crate) fn dup(fd: i32) -> i32 {
    let ret: i32;
    unsafe {
        asm!(
            "syscall",

            in("rax") 32,
            in("edi") fd,

            lateout("eax") ret,
        );
    }

    ret
}

pub(crate) fn dup2(from: i32, to: i32) -> i32 {
    let ret: i32;
    unsafe {
        asm!(
            "syscall",

            in("rax") 32,
            in("edi") from,
            in("esi") to,

            lateout("eax") ret,
        );
    }

    ret
}

pub(crate) fn unlink(path: *const u8) -> i32 {
    let ret: i32;
    unsafe {
        asm!(
            "syscall",

            in("rax") 87,
            in("rdi") path,

            lateout("eax") ret,
        );
    }

    ret
}

pub(crate) fn exit(code: i32) -> i32 {
    let ret: i32;
    unsafe {
        asm!(
            "syscall",

            in("rax") 60,
            in("edi") code,
            lateout("eax") ret,
        );
    }

    ret
}

pub(crate) fn getpid() -> i32 {
    let ret: i32;
    unsafe {
        asm!(
            "syscall",

            in("rax") 30,
            lateout("eax") ret,
        );
    }

    ret
}

pub(crate) fn fork() -> i32 {
    let ret: i32;
    unsafe {
        asm!(
            "syscall",

            in("rax") 57,
            lateout("eax") ret,
        );
    }

    ret
}

pub(crate) fn vfork() -> i32 {
    let ret: i32;
    unsafe {
        asm!(
            "syscall",

            in("rax") 58,
            lateout("eax") ret,
        );
    }

    ret
}

pub(crate) fn mmap(
    addr: *mut u8,
    len: usize,
    prot: usize,
    flags: usize,
    fd: usize,
    off: usize,
) -> i64 {
    let ret: i64;

    unsafe {
        asm!(
            "syscall",

            in("rax") 9,
            in("rdi") addr,
            in("rsi") len,
            in("rdx") prot,
            in("r10") flags,
            in("r8") fd,
            in("r9") off,

            lateout("eax") ret,
        );
    }

    ret
}

pub(crate) fn munmap(addr: *const u8, len: usize) -> i32 {
    let ret: i32;

    unsafe {
        asm!(
            "syscall",

            in("rax") 11,
            in("rdi") addr,
            in("rsi") len,

            lateout("eax") ret,
        );
    }

    ret
}

pub(crate) fn lseek(fd: i32, offset: usize, origin: u32) -> isize {
    let ret: isize;

    unsafe {
        asm!(
            "syscall",

            in("rax") 8,
            in("rdi") fd,
            in("rsi") offset,
            in("rdx") origin,

            lateout("rax") ret,
        );
    }

    ret
}

#[cfg(feature = "socket")]
pub(crate) fn socket(family: u32, ty: u32, prot: u32) -> i32 {
    let ret: i32;
    unsafe {
        asm!(
            "syscall",

            in("rax") 41,
            in("edi") family,
            in("esi") ty,
            in("edx") prot,

            lateout("eax") ret,
        );
    }

    ret
}

#[cfg(feature = "socket")]
pub(crate) fn bind(fd: i32, addr: *const (), len: usize) -> i32 {
    let ret: i32;
    unsafe {
        asm!(
            "syscall",

            in("rax") 49,
            in("edi") fd,
            in("esi") addr,
            in("edx") len,

            lateout("eax") ret,
        );
    }

    ret
}

#[cfg(feature = "socket")]
pub(crate) fn listen(fd: i32, backlog: u32) -> i32 {
    let ret: i32;
    unsafe {
        asm!(
            "syscall",

            in("rax") 50,
            in("edi") fd,
            in("esi") backlog,

            lateout("rax") ret,
        );
    }

    ret
}

#[cfg(feature = "socket")]
pub(crate) fn accept(fd: i32, addr: *mut (), len: *mut usize) -> i32 {
    let ret: i32;
    unsafe {
        asm!(
            "syscall",

            in("rax") 43,
            in("rdi") fd,
            in("rsi") addr,
            in("rdx") len,

            lateout("rax") ret,
        );
    }

    ret
}

#[cfg(feature = "socket")]
pub(crate) fn connect(fd: i32, addr: *const (), len: usize) -> i32 {
    let ret: i32;
    unsafe {
        asm!(
            "syscall",

            in("rax") 42,
            in("rdi") fd,
            in("rsi") addr,
            in("rdx") len,

            lateout("rax") ret,
        );
    }

    ret
}
