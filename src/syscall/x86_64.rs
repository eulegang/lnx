use core::arch::asm;
use lnx_syscall::syscall;

syscall!(0, read -> i32, edi = fd: i32, rsi = addr: *mut u8, rdx = len: usize);
syscall!(1, write -> i32, edi = fd: i32, rsi = addr: *const u8, rdx = len: usize);
syscall!(2, open -> i32, rdi = path: *const u8, rsi = flags: u32, rdx = perms: u32);
syscall!(3, close -> i32, edi = fd: i32);

syscall!(8, lseek -> isize, rdi = fd: i32, rsi = offset: usize, rdx = origin: u32);

syscall!(9, mmap -> i64, 
         rdi = addr: *mut u8,
         rsi = len: usize,
         rdx = prot: usize,
         r10 = flags: usize,
         r8 = fd: usize,
         r9 = off: usize);

syscall!(11, munmap -> i32, rdi = addr: *const u8, rsi = len: usize);

syscall!(30, getpid -> i32);

syscall!(32, dup -> i32, edi = fd: i32);
syscall!(33, dup2 -> i32, edi = from: i32, esi = to: i32);

syscall!(57, fork -> i32);
syscall!(58, vfork -> i32);

syscall!(87, unlink -> i32, rdi = path: *const u8);

syscall!(293, pipe2 -> i32, rdx = fds: &mut [i32; 2], esi = flags: u32);

#[cfg(feature = "socket")]
pub(crate) use socket::*;

#[cfg(feature = "socket")]
mod socket {
    use super::{asm, syscall};

    syscall!(41, socket -> i32, edi = family: u32, esi = ty: u32, edx = prot: u32);
    syscall!(42, connect -> i32, edi = fd: i32, rsi = addr: *const (), rdx = len: usize);
    syscall!(43, accept -> i32, edi = fd: i32, rsi = addr: *mut (), rdx = len: *mut usize);
    syscall!(49, bind -> i32, edi = fd: i32, rsi = addr: *const(), edx = len: usize);
    syscall!(50, listen -> i32, edi = fd: i32, esi = backlog: u32);
}

pub(crate) fn exit(code: i32) -> ! {
    unsafe {
        asm!(
            "syscall",

            in("rax") 60,
            in("edi") code,
            options(noreturn),
        );
    }
}
