use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::result::Result;
use std::result::Result::{Ok, Err};
use std::option::Option::{None, Some};
use std::str::FromStr;

use iprange::{IpAddrRange, IpAddrRangeError};


fn ipv4_to_u32(ip: &Ipv4Addr) -> u32 {
    ip.octets().iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (count, bits)| {
            acc | ((*bits as u32) << (count * 8))
        })
}

fn number_of_common_prefix_bits(start: &Ipv4Addr, end: &Ipv4Addr) -> usize {
    let ip1 = ipv4_to_u32(start);
    let ip2 = ipv4_to_u32(end);
    (ip1 ^ ip2).leading_zeros() as usize
}

fn number_of_common_postfix_bits(start: &Ipv4Addr, end: &Ipv4Addr) -> usize {
    let ip1 = ipv4_to_u32(start);
    let ip2 = ipv4_to_u32(end);
    (ip1 ^ ip2).trailing_zeros() as usize
}

fn number_of_diff_postfix_bits(start: &Ipv4Addr, end: &Ipv4Addr) -> usize {
    let ip1 = ipv4_to_u32(start);
    let ip2 = ipv4_to_u32(end);
    (!(ip1 ^ ip2)).trailing_zeros() as usize
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct IpAddrRangeV4 {
	address: Ipv4Addr,
	mask: u8,
}

impl IpAddrRangeV4 {
	pub fn new(address: Ipv4Addr, mask: u8) -> IpAddrRangeV4 {
		assert!(mask <= 32);
		IpAddrRangeV4 {
			address: address,
			mask: mask,
		}
	}

	// TODO: work
	pub fn from_range(start: Ipv4Addr, end: Ipv4Addr) -> Result<IpAddrRangeV4, IpAddrRangeError> {
		if start == end {
			return Ok(IpAddrRangeV4 {
				address: start,
				mask: 32,
			});
		}
        let start_num = ipv4_to_u32(&start);
        let end_num = ipv4_to_u32(&end);
        if start_num > end_num {
            panic!("Can not summarize_range start > end (s:{:?} e:{:?})", start, end);
        }
		unimplemented!();
	}

	pub fn address(&self) -> Ipv4Addr {
		self.address
	}

	pub fn mask(&self) -> u8 {
		self.mask
	}
}

impl ToString for IpAddrRangeV4 {
    fn to_string(&self) -> String {
    	format!("{}/{}", self.address, self.mask)
    }
}

impl FromStr for IpAddrRangeV4 {
	type Err = IpAddrRangeError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let split_point = s.find('/').ok_or(IpAddrRangeError::ParseError)?;
		let (address_str, _) = s.split_at(split_point);
		let (_, mask_str) = s.split_at(split_point + 1);
		let address = Ipv4Addr::from_str(address_str).map_err(|_| IpAddrRangeError::ParseError)?;
		let mask = u8::from_str(mask_str).map_err(|_| IpAddrRangeError::ParseError)?;
		Ok(IpAddrRangeV4::new(address, mask))
	}
}

#[cfg(test)]
mod tests {
	use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
	use std::str::FromStr;

	use super::*;

    fn ipv4(s: &str) -> Ipv4Addr {
        Ipv4Addr::from_str(s).unwrap()
    }

    #[test]
    fn ipv4_to_u32_zero() {
        let ip = ipv4("0.0.0.0");
        assert_eq!(ipv4_to_u32(&ip), 0x000000);
    }

    #[test]
    fn ipv4_to_u32_le_be() {
        let ip1 = ipv4("0.0.255.255");
        assert_eq!(ipv4_to_u32(&ip1), 0x0000ffff);
        let ip2 = ipv4("255.255.0.0");
        assert_eq!(ipv4_to_u32(&ip2), 0xffff0000);
    }

    #[test]
    fn ipv4_to_u32_ff() {
        let ip = ipv4("255.255.255.255");
        assert_eq!(ipv4_to_u32(&ip), 0xffffffff);
    }

    #[test]
    fn ipv4_to_u32_localhost() {
        let ip = ipv4("127.0.0.1");
        assert_eq!(ipv4_to_u32(&ip), 0x7f000001);
    }

    #[test]
    fn number_of_common_diff_pre_postfix_bits_same_address() {
        let ip = ipv4("0.0.0.0");
        assert_eq!(number_of_common_prefix_bits(&ip, &ip), 32);
        assert_eq!(number_of_common_postfix_bits(&ip, &ip), 32);

        assert_eq!(number_of_diff_postfix_bits(&ip, &ip), 0);
    }

    #[test]
    fn number_of_common_diff_pre_postfix_bits_no_common_prefix() {
        let ip1 = ipv4("0.0.0.0");
        let ip2 = ipv4("255.255.255.255");
        assert_eq!(number_of_common_prefix_bits(&ip1, &ip2), 0);
        assert_eq!(number_of_common_postfix_bits(&ip1, &ip2), 0);

        assert_eq!(number_of_diff_postfix_bits(&ip1, &ip2), 32);
    }

    #[test]
    fn number_of_common_diff_pre_postfix_bits_first() {
        let ip1 = ipv4("127.255.255.255");
        let ip2 = ipv4("255.255.255.255");
        assert_eq!(number_of_common_prefix_bits(&ip1, &ip2), 0);
        assert_eq!(number_of_common_postfix_bits(&ip1, &ip2), 31);

        assert_eq!(number_of_diff_postfix_bits(&ip1, &ip2), 0);
    }

        #[test]
    fn number_of_common_diff_pre_postfix_bits_last() {
        let ip1 = ipv4("0.0.0.0");
        let ip2 = ipv4("0.0.0.1");
        assert_eq!(number_of_common_prefix_bits(&ip1, &ip2), 31);
        assert_eq!(number_of_common_postfix_bits(&ip1, &ip2), 0);

        assert_eq!(number_of_diff_postfix_bits(&ip1, &ip2), 1);
    }

        #[test]
    fn number_of_common_diff_pre_postfix_bits_mid() {
        let ip1 = ipv4("192.168.1.0");
        let ip2 = ipv4("192.168.2.0");
        assert_eq!(number_of_common_prefix_bits(&ip1, &ip2), 22);
        assert_eq!(number_of_common_postfix_bits(&ip1, &ip2), 8);

        assert_eq!(number_of_diff_postfix_bits(&ip1, &ip2), 0);
    }

    #[test]
    fn from_range_same_address() {
        let ip1 = ipv4("127.0.0.1");
        let ip2 = ipv4("127.0.0.1");
        let result = IpAddrRange::from_range(IpAddr::V4(ip1), IpAddr::V4(ip2));
        assert!(result.is_ok());
        let range = result.unwrap();
        match range {
            IpAddrRange::V4(range_v4) => {
                assert_eq!(range_v4.to_string(), String::from("127.0.0.1/32"));
            },
            _ => assert!(false),
        }
    }

	#[test]
	fn from_range_simple_netmask() {
		let ip1 = ipv4("127.0.0.0");
		let ip2 = ipv4("127.0.0.255");
		let result = IpAddrRange::from_range(IpAddr::V4(ip1), IpAddr::V4(ip2));
		assert!(result.is_ok());
		let range = result.unwrap();
		match range {
		    IpAddrRange::V4(range_v4) => {
		    	assert_eq!(range_v4.to_string(), String::from("127.0.0.0/24"));
		    },
		    _ => assert!(false),
		}
	}
}