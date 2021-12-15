pub(crate) fn pipe2(fds: &mut [i32; 2], flags: u32) -> i32 {
    let ret: i32;
    unsafe {
        asm!(
            "mov rax, {syscall}",
            "syscall",

            syscall = const 293,

            in("rdi") fds,
            in("esi") flags,
            out("eax") ret,
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
            out("eax") ret,
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

            out("eax") ret,
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

            out("eax") ret,
        );
    }

    ret
}
