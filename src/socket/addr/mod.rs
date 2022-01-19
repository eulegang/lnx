mod ip;

pub use ip::{in4_addr, IPv4};

/// NEVER dyn this trait
pub trait socket_addr {
    const FAMILY: u16;
    const SIZE: usize;
}
