use core::num::NonZeroU32;
use core::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub struct Errno {
    err: NonZeroU32,
}

impl Errno {
    pub fn new(result: i32) -> Result<u32, Errno> {
        if result < 0 {
            let err = unsafe { NonZeroU32::new_unchecked((-result) as u32) };

            Err(Errno { err })
        } else {
            Ok(result as u32)
        }
    }
}

impl Display for Errno {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let idx = self.err.get() as usize;
        if idx >= ERRNO_STR.len() {
            write!(fmt, "unknown errno")
        } else {
            write!(fmt, "{}", &ERRNO_STR[idx])
        }

    }
}

const ERRNO_STR: &[&str] = &[
    "",
    "Operation not permitted",
    "No such file or directory",
    "No such process",
    "Interrupted system call",
    "Input/output error",
    "No such device or address",
    "Argument list too long",
    "Exec format error",
    "Bad file descriptor",
    "No child processes",
    "Resource temporarily unavailable",
    "Cannot allocate memory",
    "Permission denied",
    "Bad address",
    "Block device required",
    "Device or resource busy",
    "File exists",
    "Invalid cross-device link",
    "No such device",
    "Not a directory",
    "Is a directory",
    "Invalid argument",
    "Too many open files in system",
    "Too many open files",
    "Inappropriate ioctl for device",
    "Text file busy",
    "File too large",
    "No space left on device",
    "Illegal seek",
    "Read-only file system",
    "Too many links",
    "Broken pipe",
    "Numerical argument out of domain",
    "Numerical result out of range",
    "Resource deadlock avoided",
    "File name too long",
    "No locks available",
    "Function not implemented",
    "Directory not empty",
    "Too many levels of symbolic links",
    "Resource temporarily unavailable",
    "No message of desired type",
    "Identifier removed",
    "Channel number out of range",
    "Level 2 not synchronized",
    "Level 3 halted",
    "Level 3 reset",
    "Link number out of range",
    "Protocol driver not attached",
    "No CSI structure available",
    "Level 2 halted",
    "Invalid exchange",
    "Invalid request descriptor",
    "Exchange full",
    "No anode",
    "Invalid request code",
    "Invalid slot",
    "Resource deadlock avoided",
    "Bad font file format",
    "Device not a stream",
    "No data available",
    "Timer expired",
    "Out of streams resources",
    "Machine is not on the network",
    "Package not installed",
    "Object is remote",
    "Link has been severed",
    "Advertise error",
    "Srmount error",
    "Communication error on send",
    "Protocol error",
    "Multihop attempted",
    "RFS specific error",
    "Bad message",
    "Value too large for defined data type",
    "Name not unique on network",
    "File descriptor in bad state",
    "Remote address changed",
    "Can not access a needed shared library",
    "Accessing a corrupted shared library",
    ".lib section in a.out corrupted",
    "Attempting to link in too many shared libraries",
    "Cannot exec a shared library directly",
    "Invalid or incomplete multibyte or wide character",
    "Interrupted system call should be restarted",
    "Streams pipe error",
    "Too many users",
    "Socket operation on non-socket",
    "Destination address required",
    "Message too long",
    "Protocol wrong type for socket",
    "Protocol not available",
    "Protocol not supported",
    "Socket type not supported",
    "Operation not supported",
    "Protocol family not supported",
    "Address family not supported by protocol",
    "Address already in use",
    "Cannot assign requested address",
    "Network is down",
    "Network is unreachable",
    "Network dropped connection on reset",
    "Software caused connection abort",
    "Connection reset by peer",
    "No buffer space available",
    "Transport endpoint is already connected",
    "Transport endpoint is not connected",
    "Cannot send after transport endpoint shutdown",
    "Too many references: cannot splice",
    "Connection timed out",
    "Connection refused",
    "Host is down",
    "No route to host",
    "Operation already in progress",
    "Operation now in progress",
    "Stale file handle",
    "Structure needs cleaning",
    "Not a XENIX named type file",
    "No XENIX semaphores available",
    "Is a named type file",
    "Remote I/O error",
    "Disk quota exceeded",
    "No medium found",
    "Wrong medium type",
    "Operation canceled",
    "Required key not available",
    "Key has expired",
    "Key has been revoked",
    "Key was rejected by service",
    "Owner died",
    "State not recoverable",
    "Operation not possible due to RF-kill",
    "Memory page has hardware error",
];

impl Errno {
    pub const EPERM: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(1) } };
    pub const ENOENT: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(2) } };
    pub const ESRCH: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(3) } };
    pub const EINTR: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(4) } };
    pub const EIO: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(5) } };
    pub const ENXIO: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(6) } };
    pub const E2BIG: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(7) } };
    pub const ENOEXEC: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(8) } };
    pub const EBADF: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(9) } };
    pub const ECHILD: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(10) } };
    pub const EAGAIN: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(11) } };
    pub const ENOMEM: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(12) } };
    pub const EACCES: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(13) } };
    pub const EFAULT: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(14) } };
    pub const ENOTBLK: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(15) } };
    pub const EBUSY: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(16) } };
    pub const EEXIST: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(17) } };
    pub const EXDEV: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(18) } };
    pub const ENODEV: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(19) } };
    pub const ENOTDIR: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(20) } };
    pub const EISDIR: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(21) } };
    pub const EINVAL: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(22) } };
    pub const ENFILE: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(23) } };
    pub const EMFILE: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(24) } };
    pub const ENOTTY: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(25) } };
    pub const ETXTBSY: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(26) } };
    pub const EFBIG: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(27) } };
    pub const ENOSPC: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(28) } };
    pub const ESPIPE: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(29) } };
    pub const EROFS: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(30) } };
    pub const EMLINK: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(31) } };
    pub const EPIPE: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(32) } };
    pub const EDOM: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(33) } };
    pub const ERANGE: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(34) } };
    pub const EDEADLK: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(35) } };
    pub const ENAMETOOLONG: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(36) } };
    pub const ENOLCK: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(37) } };
    pub const ENOSYS: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(38) } };
    pub const ENOTEMPTY: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(39) } };
    pub const ELOOP: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(40) } };
    pub const EWOULDBLOCK: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(11) } };
    pub const ENOMSG: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(42) } };
    pub const EIDRM: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(43) } };
    pub const ECHRNG: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(44) } };
    pub const EL2NSYNC: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(45) } };
    pub const EL3HLT: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(46) } };
    pub const EL3RST: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(47) } };
    pub const ELNRNG: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(48) } };
    pub const EUNATCH: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(49) } };
    pub const ENOCSI: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(50) } };
    pub const EL2HLT: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(51) } };
    pub const EBADE: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(52) } };
    pub const EBADR: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(53) } };
    pub const EXFULL: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(54) } };
    pub const ENOANO: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(55) } };
    pub const EBADRQC: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(56) } };
    pub const EBADSLT: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(57) } };
    pub const EDEADLOCK: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(35) } };
    pub const EBFONT: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(59) } };
    pub const ENOSTR: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(60) } };
    pub const ENODATA: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(61) } };
    pub const ETIME: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(62) } };
    pub const ENOSR: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(63) } };
    pub const ENONET: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(64) } };
    pub const ENOPKG: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(65) } };
    pub const EREMOTE: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(66) } };
    pub const ENOLINK: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(67) } };
    pub const EADV: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(68) } };
    pub const ESRMNT: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(69) } };
    pub const ECOMM: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(70) } };
    pub const EPROTO: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(71) } };
    pub const EMULTIHOP: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(72) } };
    pub const EDOTDOT: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(73) } };
    pub const EBADMSG: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(74) } };
    pub const EOVERFLOW: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(75) } };
    pub const ENOTUNIQ: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(76) } };
    pub const EBADFD: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(77) } };
    pub const EREMCHG: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(78) } };
    pub const ELIBACC: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(79) } };
    pub const ELIBBAD: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(80) } };
    pub const ELIBSCN: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(81) } };
    pub const ELIBMAX: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(82) } };
    pub const ELIBEXEC: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(83) } };
    pub const EILSEQ: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(84) } };
    pub const ERESTART: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(85) } };
    pub const ESTRPIPE: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(86) } };
    pub const EUSERS: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(87) } };
    pub const ENOTSOCK: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(88) } };
    pub const EDESTADDRREQ: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(89) } };
    pub const EMSGSIZE: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(90) } };
    pub const EPROTOTYPE: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(91) } };
    pub const ENOPROTOOPT: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(92) } };
    pub const EPROTONOSUPPORT: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(93) } };
    pub const ESOCKTNOSUPPORT: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(94) } };
    pub const EOPNOTSUPP: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(95) } };
    pub const EPFNOSUPPORT: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(96) } };
    pub const EAFNOSUPPORT: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(97) } };
    pub const EADDRINUSE: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(98) } };
    pub const EADDRNOTAVAIL: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(99) } };
    pub const ENETDOWN: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(100) } };
    pub const ENETUNREACH: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(101) } };
    pub const ENETRESET: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(102) } };
    pub const ECONNABORTED: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(103) } };
    pub const ECONNRESET: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(104) } };
    pub const ENOBUFS: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(105) } };
    pub const EISCONN: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(106) } };
    pub const ENOTCONN: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(107) } };
    pub const ESHUTDOWN: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(108) } };
    pub const ETOOMANYREFS: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(109) } };
    pub const ETIMEDOUT: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(110) } };
    pub const ECONNREFUSED: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(111) } };
    pub const EHOSTDOWN: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(112) } };
    pub const EHOSTUNREACH: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(113) } };
    pub const EALREADY: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(114) } };
    pub const EINPROGRESS: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(115) } };
    pub const ESTALE: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(116) } };
    pub const EUCLEAN: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(117) } };
    pub const ENOTNAM: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(118) } };
    pub const ENAVAIL: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(119) } };
    pub const EISNAM: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(120) } };
    pub const EREMOTEIO: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(121) } };
    pub const EDQUOT: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(122) } };
    pub const ENOMEDIUM: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(123) } };
    pub const EMEDIUMTYPE: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(124) } };
    pub const ECANCELED: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(125) } };
    pub const ENOKEY: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(126) } };
    pub const EKEYEXPIRED: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(127) } };
    pub const EKEYREVOKED: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(128) } };
    pub const EKEYREJECTED: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(129) } };
    pub const EOWNERDEAD: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(130) } };
    pub const ENOTRECOVERABLE: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(131) } };
    pub const ERFKILL: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(132) } };
    pub const EHWPOISON: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(133) } };
    pub const ENOTSUP: Errno = Errno { err: unsafe { NonZeroU32::new_unchecked(95) } };
}
