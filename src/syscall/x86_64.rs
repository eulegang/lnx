use lnx_syscall::syscall;

syscall!(0, pub(crate) fn read(fd: i32, addr: *mut u8, len: usize) -> i32);
syscall!(1, pub(crate) fn write(fd: i32, addr: *const u8, len: usize) -> i32);
syscall!(2, pub(crate) fn open(path: *const u8, flags: u32, perms: u32) -> i32);
syscall!(3, pub(crate) fn close(fd: i32) -> i32);

syscall!(8, pub(crate) fn lseek(fd: i32, offset: usize, origin: u32) -> isize);

syscall!(9, pub(crate) fn mmap(addr: *mut u8, len: usize, prot: usize, flags: usize, fd: usize, off: usize) -> i64);

syscall!(11, pub(crate) fn munmap(addr: *const u8, len: usize) -> i32);

syscall!(30, pub(crate) fn getpid() -> i32);

syscall!(32, pub(crate) fn dup(fd: i32) -> i32);
syscall!(33, pub(crate) fn dup2(from: i32, to: i32) -> i32);

syscall!(57, pub(crate) fn fork() -> i32);
syscall!(58, pub(crate) fn vfork() -> i32);

syscall!(60, pub(crate) fn exit(code: i32) -> !);

syscall!(87, pub(crate) fn unlink(path: *const u8) -> i32);

syscall!(293, pub(crate) fn pipe2(fds: &mut [i32; 2], flags: u32) -> i32);

#[cfg(feature = "socket")]
pub(crate) use socket::*;

#[cfg(feature = "socket")]
mod socket {
    use super::syscall;

    syscall!(41, pub(crate) fn socket(family: u32, ty: u32, prot: u32) -> i32);
    syscall!(42, pub(crate) fn connect(fd: i32, addr: *const(), len: usize) -> i32);
    syscall!(43, pub(crate) fn accept(fd: i32, addr: *mut(), len: *mut usize) -> i32);
    syscall!(49, pub(crate) fn bind(fd: i32, addr: *const(), len: usize) -> i32);
    syscall!(50, pub(crate) fn listen(fd: i32, backlog: u32) -> i32);

    syscall!(54, pub(crate) fn setsockopt(fd: i32, level: u32, optname: u32, optval: *const u8, len: usize) -> i32);
    syscall!(55, pub(crate) fn getsockopt(fd: i32, level: u32, optname: u32, optval: *mut u8, len: *mut usize) -> i32);
}
