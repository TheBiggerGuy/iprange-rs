use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::result::Result;
use std::result::Result::{Ok, Err};
use std::option::Option::{None, Some};
use std::str::FromStr;

use iprange::{IpAddrRange, IpAddrRangeError};

#[derive(Debug)]
pub struct IpAddrRangeV6 {
	address: Ipv6Addr,
	mask: u8,
}

impl IpAddrRangeV6 {
	pub fn new(address: Ipv6Addr, mask: u8) -> IpAddrRangeV6 {
		assert!(mask <= 128);
		IpAddrRangeV6 {
			address: address,
			mask: mask,
		}
	}

	// TODO: work
	pub fn from_range(start: Ipv6Addr, end: Ipv6Addr) -> Result<IpAddrRangeV6, IpAddrRangeError> {
		if start == end {
			return Ok(IpAddrRangeV6 {
				address: start,
				mask: 128,
			});
		}
		unimplemented!();
	}

	pub fn address(&self) -> Ipv6Addr {
		self.address
	}

	pub fn mask(&self) -> u8 {
		self.mask
	}
}

impl ToString for IpAddrRangeV6 {
    fn to_string(&self) -> String {
    	format!("{}/{}", self.address, self.mask)
    }
}

#[cfg(test)]
mod tests {
	use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
	use std::str::FromStr;

	use super::*;

    #[test]
    fn from_range_same_address() {
        let ip1 = Ipv6Addr::from_str("::1").unwrap();
        let ip2 = Ipv6Addr::from_str("::1").unwrap();
        let result = IpAddrRange::from_range(IpAddr::V6(ip1), IpAddr::V6(ip2));
        assert!(result.is_ok());
        let range = result.unwrap();
        match range {
            IpAddrRange::V6(range_v6) => {
                assert_eq!(range_v6.to_string(), String::from("::1/128"));
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn from_range_simple_netmask() {
        let ip1 = Ipv6Addr::from_str("::0").unwrap();
        let ip2 = Ipv6Addr::from_str("::ff").unwrap();
        let result = IpAddrRange::from_range(IpAddr::V6(ip1), IpAddr::V6(ip2));
        assert!(result.is_ok());
        let range = result.unwrap();
        match range {
            IpAddrRange::V6(range_v6) => {
                assert_eq!(range_v6.to_string(), String::from("::0/112"));
            },
            _ => assert!(false),
        }
    }
}