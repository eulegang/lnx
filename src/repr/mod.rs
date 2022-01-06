const DIGITS: [u8; 16] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F',
];

pub trait Repr {
    fn repr(&self, buf: &mut [u8]) -> Option<usize>;

    fn hint(&self) -> Option<core::num::NonZeroUsize> {
        None
    }
}

impl Repr for usize {
    #[cfg(target_pointer_width = "64")]
    fn hint(&self) -> Option<core::num::NonZeroUsize> {
        Some(unsafe { core::num::NonZeroUsize::new_unchecked(18) })
    }

    #[cfg(target_pointer_width = "64")]
    fn repr(&self, buf: &mut [u8]) -> Option<usize> {
        if buf.len() < 18 {
            return None;
        }

        buf[0] = b'0';
        buf[1] = b'x';

        buf[2] = DIGITS[(self >> 60) & 0x0F];
        buf[3] = DIGITS[(self >> 56) & 0x0F];
        buf[4] = DIGITS[(self >> 52) & 0x0F];
        buf[5] = DIGITS[(self >> 48) & 0x0F];

        buf[6] = DIGITS[(self >> 44) & 0x0F];
        buf[7] = DIGITS[(self >> 40) & 0x0F];
        buf[8] = DIGITS[(self >> 36) & 0x0F];
        buf[9] = DIGITS[(self >> 32) & 0x0F];

        buf[10] = DIGITS[(self >> 28) & 0x0F];
        buf[11] = DIGITS[(self >> 24) & 0x0F];
        buf[12] = DIGITS[(self >> 20) & 0x0F];
        buf[13] = DIGITS[(self >> 16) & 0x0F];

        buf[14] = DIGITS[(self >> 12) & 0x0F];
        buf[15] = DIGITS[(self >> 8) & 0x0F];
        buf[16] = DIGITS[(self >> 4) & 0x0F];
        buf[17] = DIGITS[self & 0x0F];

        Some(18)
    }

    #[cfg(target_pointer_width = "32")]
    fn hint(&self) -> Option<core::num::NonZeroUsize> {
        Some(unsafe { core::num::NonZeroUsize::new_unchecked(10) })
    }

    #[cfg(target_pointer_width = "32")]
    fn repr(&self, buf: &mut [u8]) -> Option<usize> {
        if buf.len() < 10 {
            return None;
        }

        buf[0] = b'0';
        buf[1] = b'x';

        buf[2] = DIGITS[(self >> 28) & 0x0F];
        buf[3] = DIGITS[(self >> 24) & 0x0F];
        buf[4] = DIGITS[(self >> 20) & 0x0F];
        buf[5] = DIGITS[(self >> 16) & 0x0F];

        buf[6] = DIGITS[(self >> 12) & 0x0F];
        buf[7] = DIGITS[(self >> 8) & 0x0F];
        buf[8] = DIGITS[(self >> 4) & 0x0F];
        buf[9] = DIGITS[self & 0x0F];

        Some(10)
    }

    #[cfg(target_pointer_width = "16")]
    fn hint(&self) -> Option<core::num::NonZeroUsize> {
        Some(unsafe { core::num::NonZeroUsize::new_unchecked(6) })
    }

    #[cfg(target_pointer_width = "16")]
    fn repr(&self, buf: &mut [u8]) -> Option<usize> {
        if buf.len() < 6 {
            return None;
        }

        buf[0] = b'0';
        buf[1] = b'x';

        buf[2] = DIGITS[(self >> 12) & 0x0F];
        buf[3] = DIGITS[(self >> 8) & 0x0F];
        buf[4] = DIGITS[(self >> 4) & 0x0F];
        buf[5] = DIGITS[self & 0x0F];

        Some(6)
    }
}
