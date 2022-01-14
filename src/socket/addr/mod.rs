mod ip;

pub use ip::{IPv4, in4_addr};

/// NEVER dyn this trait
pub trait socket_addr {
    const FAMILY: u16;
    const SIZE: usize;
}
