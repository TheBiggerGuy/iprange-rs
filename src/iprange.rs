use std::fmt;
use std::net::{IpAddr, AddrParseError};
use std::result::Result::{self, Ok, Err};
use std::num::ParseIntError;

use ipv4::IpAddrRangeV4;
use ipv6::IpAddrRangeV6;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IpAddrRangeError {
    MixedIpVersions,
    ParseError,
    InvalidNetworkAddress,
}

impl From<AddrParseError> for IpAddrRangeError {
    fn from(_: AddrParseError) -> IpAddrRangeError {
        IpAddrRangeError::ParseError
    }
}

impl From<ParseIntError> for IpAddrRangeError {
    fn from(_: ParseIntError) -> IpAddrRangeError {
        IpAddrRangeError::ParseError
    }
}

#[derive(Debug)]
pub enum IpAddrRange {
    V4(IpAddrRangeV4),
    V6(IpAddrRangeV6),
}

impl IpAddrRange {
    pub fn from_range(start: IpAddr, end: IpAddr) -> Result<IpAddrRange, IpAddrRangeError> {
        match (start, end) {
            (IpAddr::V4(startv4), IpAddr::V4(endv4)) => {
                Ok(IpAddrRange::V4(IpAddrRangeV4::from_range(startv4, endv4)?))
            }
            (IpAddr::V6(startv6), IpAddr::V6(endv6)) => {
                Ok(IpAddrRange::V6(IpAddrRangeV6::from_range(startv6, endv6)?))
            }
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

/*
impl ToString for IpAddrRange {
    fn to_string(&self) -> String {
        match *self {
            IpAddrRange::V4(r) => r.to_string(),
            IpAddrRange::V6(r) => r.to_string(),
        }
    }
}
*/

impl fmt::Display for IpAddrRange {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IpAddrRange::V4(ref r) => r.fmt(fmt),
            IpAddrRange::V6(ref r) => r.fmt(fmt),
        }
    }
}


/*
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
    fn to_string_v4() {
        let range_v4 = IpAddrRangeV4::new(Ipv4Addr::from_str("127.0.0.1").unwrap(), 24);
        let range = IpAddrRange::V4(range_v4.clone());
        
        assert_eq!(range.to_string(), range_v4.to_string());
    }

    #[test]
    fn to_string_v6() {
        let range_v6 = IpAddrRangeV6::new(Ipv6Addr::from_str("::1").unwrap(), 24);
        let range = IpAddrRange::V6(range_v6.clone());
        
        assert_eq!(range.to_string(), range_v6.to_string());
    }

    /*
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
