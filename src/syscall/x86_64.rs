pub(crate) fn pipe2(fds: &mut [i32; 2], flags: u32) -> i32 {
    let ret: i32;
    unsafe {
        asm!(
            "mov rax, {syscall}",
            "syscall",

            syscall = const 293,

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
            "mov rax, {syscall}",
            "syscall",

            syscall = const 3,

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
            "mov rax, {syscall}",
            "syscall",

            syscall = const 0,

            in("edi") fd,
            in("rsi") ptr,
            in("rdx") len,

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
            "mov rax, {syscall}",
            "syscall",

            syscall = const 1,

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
