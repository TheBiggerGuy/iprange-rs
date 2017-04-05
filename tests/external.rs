extern crate env_logger;
extern crate iprange;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use iprange::IpAddrRange;

#[test]
fn smoke_v4() {
    let _ = env_logger::init();

    let address_start = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 0));
    let address_end = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 255));

    let range = IpAddrRange::from_range(address_start, address_end).unwrap();
    assert!(range.is_ipv4());
    assert_eq!(range.to_string(), String::from("192.168.0.0/24"));
}

#[test]
fn smoke_v6() {
    let _ = env_logger::init();

    let address_start = IpAddr::V6(Ipv6Addr::new(0x2001, 0, 0, 0, 0, 0, 0, 0));
    let address_end = IpAddr::V6(Ipv6Addr::new(0x2001, 0, 0, 0, 0xffff, 0xffff, 0xffff, 0xffff));

    let range = IpAddrRange::from_range(address_start, address_end).unwrap();
    assert!(range.is_ipv6());
    assert_eq!(range.to_string(), String::from("2001::/64"));
}
