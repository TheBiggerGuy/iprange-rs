#![feature(i128_type)]
#![feature(test)]

extern crate log;
    extern crate test;

mod iprange;
pub use iprange::{IpAddrRange, IpAddrRangeError};

mod ipv4;
pub use ipv4::IpAddrRangeV4;

mod ipv6;
pub use ipv6::IpAddrRangeV6;

mod bits;
