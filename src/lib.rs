#![feature(i128_type)]

#[macro_use]
extern crate log;

mod iprange;
pub use iprange::{IpAddrRange, IpAddrRangeError};

mod ipv4;
pub use ipv4::IpAddrRangeV4;

mod ipv6;
pub use ipv6::IpAddrRangeV6;

mod bits;
