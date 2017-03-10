use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::result::Result;
use std::result::Result::{Ok, Err};
use std::option::Option::{None, Some};
use std::str::FromStr;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum IpAddrRangeError {
	MixedIpVersions,
	ParseError,
}

#[derive(Debug)]
pub enum IpAddrRange {
	V4(IpAddrRangeV4),
	V6(IpAddrRangeV6),
}

impl IpAddrRange {
	// TODO: work
	pub fn from_range(start: IpAddr, end: IpAddr) -> Result<IpAddrRange, IpAddrRangeError> {
		match (start, end) {
            (IpAddr::V4(startv4), IpAddr::V4(endv4)) => Ok(IpAddrRange::V4(IpAddrRangeV4::from_range(startv4, endv4)?)),
            (IpAddr::V6(startv6), IpAddr::V6(endv6)) => Ok(IpAddrRange::V6(IpAddrRangeV6::from_range(startv6, endv6)?)),
            _ => Err(IpAddrRangeError::MixedIpVersions),
        }
	}

	pub fn is_ipv4(&self) -> bool {
		match *self {
            IpAddrRange::V4(_) => true,
            IpAddrRange::V6(_) => false,
        }
    }

	pub fn is_ipv6(&self) -> bool {
		match *self {
            IpAddrRange::V4(_) => false,
            IpAddrRange::V6(_) => true,
        }
    }
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

/*
fn ipv4_vec_to_u32(ip: Ipv4Addr) -> u32 {
	ipv4_vec_to_u32(ip.octets())
}

fn ipv4_vec_to_u32(octets: [u8; 4]) -> u32 {
	octets.into_iter().rev().enumerate().fold(0, |acc, (count, bits)| {
		acc | ((bits as u32) << (count * 8))
	})
}

fn number_of_common_prefix_bits(start: &IPAddress, end: &IPAddress) -> usize {
	let ip1 = ipv4_vec_to_u32(start.parts());
	let ip2 = ipv4_vec_to_u32(end.parts());
	(ip1 ^ ip2).leading_zeros() as usize
}

fn number_of_common_postfix_bits(start: &IPAddress, end: &IPAddress) -> usize {
	let ip1 = ipv4_vec_to_u32(start.parts());
	let ip2 = ipv4_vec_to_u32(end.parts());
	(ip1 ^ ip2).trailing_zeros() as usize
}

fn number_of_diff_postfix_bits(start: &IPAddress, end: &IPAddress) -> usize {
	let ip1 = ipv4_vec_to_u32(start.parts());
	let ip2 = ipv4_vec_to_u32(end.parts());
	(!(ip1 ^ ip2)).trailing_zeros() as usize
}

fn next_ip(ip: &IPAddress) -> IPAddress {
	assert!(ip.prefix().host_mask().is_zero());
	ip.from(ip.to_network() + 1, ip.prefix())
}

impl IPAddressFromStartEnd for IPAddress {
    fn summarize_range(start: &IPAddress, end: &IPAddress) -> Vec<IPAddress> {
    	if !start.prefix().host_mask().is_zero() || !end.prefix().host_mask().is_zero() {
    		panic!("TODO: support prefix s:{:?} e:{:?}", start, end);
    	}
    	if start.is_ipv6() || end.is_ipv6() {
    		panic!("TODO: support IPv6 (s:{:?} e:{:?})", start, end);
    	}
    	if start.is_ipv4() != end.is_ipv4() {
    		panic!("Can not summarize_range mixed IPv4/6 adresses (s:{:?} e:{:?})", start, end);
    	}
    	if start > end {
    		panic!("Can not summarize_range start > end (s:{:?} e:{:?})", start, end);
    	}
    	if start == end {
    		return vec![start.clone()]
    	}

    	let number_of_common_prefix_bits = number_of_common_prefix_bits(start, end);
    	let number_of_common_postfix_bits = number_of_common_postfix_bits(start, end);
    	let number_of_diff_postfix_bits = number_of_diff_postfix_bits(start, end);

    	println!("number_of_common_prefix_bits: {:?}", number_of_common_prefix_bits);
    	println!("number_of_common_postfix_bits: {:?}", number_of_common_postfix_bits);
    	println!("number_of_diff_postfix_bits: {:?}", number_of_diff_postfix_bits);
        
    	let ip_bit_size = get_address_bit_size(start);
    	println!("ip_bit_size: {:?}", ip_bit_size);


    	let differential_bit_range_size = number_of_common_prefix_bits + number_of_diff_postfix_bits;
    	println!("differential_bit_range_size: {:?}", differential_bit_range_size);

    	// test for perfect simple range match
    	if differential_bit_range_size == ip_bit_size {
    		let ranged_ip = start.change_prefix(number_of_common_prefix_bits).unwrap();
    		return vec![ranged_ip]
    	}

    	let number_of_common_prefix_bits_with_full_subnet = number_of_common_prefix_bits + number_of_common_postfix_bits;
    	println!("number_of_common_prefix_bits_with_full_subnet: {:?}", number_of_common_prefix_bits_with_full_subnet);

    	let ranged_start = start.change_prefix(number_of_common_prefix_bits_with_full_subnet);

        let result_ips = vec![ranged_start];
    	let mut working_ip = next_ip(ranged_start.last());
    	while working_ip < end {
    		result_ips.push(working_ip);
    		working_ip = next_ip(working_ip);
    	}

    	result_ips
    }
}
*/


#[cfg(test)]
mod tests {
	use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
	use std::str::FromStr;

	use super::*;

	#[test]
	fn ip_addr_range_from_str_normal() {
		let range_from_str = IpAddrRangeV4::from_str("127.0.0.1/32").unwrap();
		let expected_from_typed = IpAddrRangeV4::new(Ipv4Addr::new(127, 0, 0, 1), 32);
		assert_eq!(range_from_str, expected_from_typed);
	}

	#[test]
	fn ip_addr_range_from_str_empty() {
		let range_from_str = IpAddrRangeV4::from_str("");
		assert_eq!(range_from_str, Err(IpAddrRangeError::ParseError));
	}

	#[test]
	fn ip_addr_range_from_str_missing_mask() {
		let range_from_str = IpAddrRangeV4::from_str("127.0.0.1/");
		assert_eq!(range_from_str, Err(IpAddrRangeError::ParseError));
	}

	#[test]
	fn ip_addr_range_from_str_missing_mask_and_slash() {
		let range_from_str = IpAddrRangeV4::from_str("127.0.0.1");
		assert_eq!(range_from_str, Err(IpAddrRangeError::ParseError));
	}

	fn ip_addr_range_from_str_missing_address() {
		let range_from_str = IpAddrRangeV4::from_str("/32");
		assert_eq!(range_from_str, Err(IpAddrRangeError::ParseError));
	}

	#[test]
	fn ip_addr_range_from_range_mixedtypes() {
		let v4 = Ipv4Addr::from_str("127.0.0.1").unwrap();
		let v6 = Ipv6Addr::from_str("::1").unwrap();
		let result = IpAddrRange::from_range(IpAddr::V4(v4), IpAddr::V6(v6));
		assert!(result.is_err());
		assert_eq!(result.unwrap_err(), IpAddrRangeError::MixedIpVersions);
	}

	#[test]
	fn ip_addr_range_from_range_v4() {
		let ip1 = Ipv4Addr::from_str("127.0.0.0").unwrap();
		let ip2 = Ipv4Addr::from_str("127.0.0.255").unwrap();
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

	#[test]
	fn ip_addr_range_from_range_v6() {
		let ip1 = Ipv6Addr::from_str("::0").unwrap();
		let ip2 = Ipv6Addr::from_str("::ff").unwrap();
		let result = IpAddrRange::from_range(IpAddr::V6(ip1), IpAddr::V6(ip2));
		assert!(result.is_ok());
		let range = result.unwrap();
		match range {
		    IpAddrRange::V6(range_v6) => {
		    	assert_eq!(range_v6.to_string(), String::from("::0/24"));
		    },
		    _ => assert!(false),
		}
	}

/*
	#[test]
    fn ipv4_vec_to_u32_zero() {
    	let vec = Ipv4Addr::from_str("0.0.0.0").unwrap().parts();
    	assert_eq!(ipv4_vec_to_u32(vec), 0x000000);
    }

	#[test]
    fn ipv4_vec_to_u32_le_be() {
    	let vec1 = Ipv4Addr::from_str("0.0.255.255").unwrap().parts();
    	assert_eq!(ipv4_vec_to_u32(vec1), 0x0000ffff);
    	let vec2 = Ipv4Addr::from_str("255.255.0.0").unwrap().parts();
    	assert_eq!(ipv4_vec_to_u32(vec2), 0xffff0000);
    }

    #[test]
    fn ipv4_vec_to_u32_ff() {
    	let vec = Ipv4Addr::from_str("255.255.255.255").unwrap().parts();
    	assert_eq!(ipv4_vec_to_u32(vec), 0xffffffff);
    }

    #[test]
    fn ipv4_vec_to_u32_localhost() {
    	let vec = Ipv4Addr::from_str("127.0.0.1").unwrap().parts();
    	assert_eq!(ipv4_vec_to_u32(vec), 0x7f000001);
    }

 	#[test]
    fn number_of_common_diff_pre_postfix_bits_same_address() {
    	let ip = Ipv4Addr::from_str("0.0.0.0").unwrap();
    	assert_eq!(number_of_common_prefix_bits(&ip, &ip), 32);
    	assert_eq!(number_of_common_postfix_bits(&ip, &ip), 32);

    	assert_eq!(number_of_diff_postfix_bits(&ip, &ip), 0);
    }

    #[test]
    fn number_of_common_diff_pre_postfix_bits_no_common_prefix() {
    	let ip1 = Ipv4Addr::from_str("0.0.0.0").unwrap();
    	let ip2 = Ipv4Addr::from_str("255.255.255.255").unwrap();
    	assert_eq!(number_of_common_prefix_bits(&ip1, &ip2), 0);
    	assert_eq!(number_of_common_postfix_bits(&ip1, &ip2), 0);

    	assert_eq!(number_of_diff_postfix_bits(&ip1, &ip2), 32);
    }

    #[test]
    fn number_of_common_diff_pre_postfix_bits_first() {
    	let ip1 = Ipv4Addr::from_str("127.255.255.255").unwrap();
    	let ip2 = Ipv4Addr::from_str("255.255.255.255").unwrap();
    	assert_eq!(number_of_common_prefix_bits(&ip1, &ip2), 0);
    	assert_eq!(number_of_common_postfix_bits(&ip1, &ip2), 31);

    	assert_eq!(number_of_diff_postfix_bits(&ip1, &ip2), 0);
    }

        #[test]
    fn number_of_common_diff_pre_postfix_bits_last() {
    	let ip1 = Ipv4Addr::from_str("0.0.0.0").unwrap();
    	let ip2 = Ipv4Addr::from_str("0.0.0.1").unwrap();
    	assert_eq!(number_of_common_prefix_bits(&ip1, &ip2), 31);
    	assert_eq!(number_of_common_postfix_bits(&ip1, &ip2), 0);

    	assert_eq!(number_of_diff_postfix_bits(&ip1, &ip2), 1);
    }

        #[test]
    fn number_of_common_diff_pre_postfix_bits_mid() {
    	let ip1 = Ipv4Addr::from_str("192.168.1.0").unwrap();
    	let ip2 = Ipv4Addr::from_str("192.168.2.0").unwrap();
    	assert_eq!(number_of_common_prefix_bits(&ip1, &ip2), 22);
    	assert_eq!(number_of_common_postfix_bits(&ip1, &ip2), 8);

    	assert_eq!(number_of_diff_postfix_bits(&ip1, &ip2), 0);
    }

	#[test]
	#[should_panic]
    fn summarize_range_panic_if_start_has_range() {
    	let start = Ipv4Addr::from_str("192.168.0.0/31").unwrap();
    	let end = Ipv4Addr::from_str("192.168.0.0").unwrap();
    	IPAddress::summarize_range(&start, &end);
    }

    #[test]
	#[should_panic]
    fn summarize_range_panic_if_end_has_range() {
    	let start = Ipv4Addr::from_str("192.168.0.0").unwrap();
    	let end = Ipv4Addr::from_str("192.168.0.0/31").unwrap();
    	IPAddress::summarize_range(&start, &end);
    }

    #[test]
	#[should_panic]
    fn summarize_range_panic_if_start_ge_end() {
    	let start = Ipv4Addr::from_str("192.168.0.1").unwrap();
    	let end = Ipv4Addr::from_str("192.168.0.0").unwrap();
    	IPAddress::summarize_range(&start, &end);
    }

    #[test]
	#[should_panic]
    fn summarize_range_panic_if_mixed_types() {
    	let start = Ipv4Addr::from_str("127.0.0.1").unwrap();
    	let end = Ipv4Addr::from_str(":::1").unwrap();
    	IPAddress::summarize_range(&start, &end);
    }

	#[test]
    fn summarize_range_same_start_and_end() {
    	let ip = Ipv4Addr::from_str("192.168.0.0").unwrap();
    	let range = IPAddress::summarize_range(&ip, &ip);
    	assert_eq!(range.len(), 1);
    	let result_ip = range.get(0).unwrap();
    	assert_eq!(*result_ip, ip);
    }

    #[test]
    fn summarize_range_simple_prefix_24() {
    	let start = Ipv4Addr::from_str("192.168.0.0").unwrap();
    	let end = Ipv4Addr::from_str("192.168.0.255").unwrap();
    	let range = IPAddress::summarize_range(&start, &end);
    	assert_eq!(range.len(), 1);
    	let result_ip = range.get(0).unwrap();
    	let expected_ip = Ipv4Addr::from_str("192.168.0.0/24").unwrap();
    	assert_eq!(*result_ip, expected_ip);
    }

    #[test]
    fn summarize_range_simple_prefix_31() {
    	let start = Ipv4Addr::from_str("192.168.0.0").unwrap();
    	let end = Ipv4Addr::from_str("192.168.0.1").unwrap();
    	let range = IPAddress::summarize_range(&start, &end);
    	assert_eq!(range.len(), 1);
    	let result_ip = range.get(0).unwrap();
    	let expected_ip = Ipv4Addr::from_str("192.168.0.0/31").unwrap();
    	assert_eq!(*result_ip, expected_ip);
    }

    #[test]
    fn summarize_range_complex_31_plus1() {
    	let start = Ipv4Addr::from_str("192.168.0.0").unwrap();
    	let end = Ipv4Addr::from_str("192.168.0.2").unwrap();
    	let range = IPAddress::summarize_range(&start, &end);
    	assert_eq!(range.len(), 2);
    	let result_ip1 = range.get(0).unwrap();
    	let expected_ip1 = Ipv4Addr::from_str("192.168.0.0/31").unwrap();
    	assert_eq!(*result_ip1, expected_ip1);
    	let result_ip2 = range.get(1).unwrap();
    	let expected_ip2 = Ipv4Addr::from_str("192.168.0.2/32").unwrap();
    	assert_eq!(*result_ip2, expected_ip2);
    }
*/
}