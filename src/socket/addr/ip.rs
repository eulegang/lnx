use super::socket_addr;
use core::fmt::{self, Display, Formatter};
use core::str::FromStr;

pub(crate) const AF_INET: u16 = 2;

#[derive(Debug, PartialEq, Eq)]
pub struct in4_addr([u8; 4]);

#[repr(C)]
pub struct IPv4 {
    family: u16,
    port: [u8; 2],
    addr: in4_addr,

    _zero: [u8; 8],
}

impl socket_addr for IPv4 {
    const FAMILY: u16 = 2;
    const SIZE: usize = core::mem::size_of::<IPv4>();
}

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidAddress(&'static str);

impl FromStr for in4_addr {
    type Err = InvalidAddress;

    fn from_str(s: &str) -> Result<in4_addr, InvalidAddress> {
        let mut addr = [0u8; 4];

        let mut octet = 0u8;
        let mut char_proc = 0;
        let mut idx = 0;
        let mut chars = s.chars();

        while let Some(ch) = chars.next() {
            match ch {
                '.' => {
                    if char_proc == 0 {
                        return Err(InvalidAddress("empty octet"));
                    }

                    addr[idx] = octet;
                    idx += 1;
                    octet = 0;
                    char_proc = 0;
                }

                '0'..='9' => {
                    if let Some(o) = octet.checked_mul(10) {
                        octet = o;
                    } else {
                        return Err(InvalidAddress("overflown octet"));
                    }

                    if let Some(d) = octet.checked_add(ch as u8 - b'0') {
                        octet = d;
                    } else {
                        return Err(InvalidAddress("overflown octet"));
                    }
                }

                _ => return Err(InvalidAddress("use of invalid char")),
            }

            char_proc += 1;
        }

        if idx == 3 {
            addr[idx] = octet;
            Ok(in4_addr(addr))
        } else if idx < 3 {
            Err(InvalidAddress("underflow of octets"))
        } else {
            Err(InvalidAddress("overflow of octets"))
        }
    }
}

impl Display for in4_addr {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(
            fmt,
            "{}.{}.{}.{}",
            self.0[0], self.0[1], self.0[2], self.0[3]
        )
    }
}

impl core::fmt::Display for InvalidAddress {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(fmt, "invalid address: {}", self.0)
    }
}

impl From<[u8; 4]> for in4_addr {
    fn from(addr: [u8; 4]) -> in4_addr {
        in4_addr(addr)
    }
}

impl From<(in4_addr, u16)> for IPv4 {
    fn from((addr, port): (in4_addr, u16)) -> IPv4 {
        let family = AF_INET;
        let _zero = [0u8; 8];

        let port = port.to_be_bytes();

        IPv4 {
            family,
            port,
            addr,
            _zero,
        }
    }
}

#[test]
fn parse_v4() {
    assert_eq!("127.0.0.1".parse(), Ok(in4_addr([127, 0, 0, 1])));
    assert_eq!(
        "255.255.255.255".parse(),
        Ok(in4_addr([255, 255, 255, 255]))
    );

    assert_eq!(
        "521.0.0.1".parse::<in4_addr>(),
        Err(InvalidAddress("overflown octet"))
    );
    assert_eq!(
        "127.0.0".parse::<in4_addr>(),
        Err(InvalidAddress("underflow of octets"))
    );
    assert_eq!(
        "127.0.0.1.0".parse::<in4_addr>(),
        Err(InvalidAddress("overflow of octets"))
    );
    assert_eq!(
        "...".parse::<in4_addr>(),
        Err(InvalidAddress("empty octet"))
    );
}
