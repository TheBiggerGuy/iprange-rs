extern crate env_logger;
extern crate iprange;

use std::net::{IpAddr, Ipv4Addr};
use iprange::IpAddrRange;

#[test]
fn smoke() {
    let _ = env_logger::init();

    let address_start = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 0));
    let address_end = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 255));

    let range = IpAddrRange::from_range(address_start, address_end).unwrap();
    assert!(range.is_ipv4());
}
