use std::fmt;
use std::net::{IpAddr, AddrParseError};
use std::result::Result::{self, Ok, Err};
use std::num::ParseIntError;
use std::str::FromStr;

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

#[derive(Debug, Clone, Copy, PartialEq)]
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

impl fmt::Display for IpAddrRange {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IpAddrRange::V4(ref r) => r.fmt(fmt),
            IpAddrRange::V6(ref r) => r.fmt(fmt),
        }
    }
}

impl FromStr for IpAddrRange {
    type Err = IpAddrRangeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split_point = s.find('/').ok_or(IpAddrRangeError::ParseError)?;
        let (address_str, _) = s.split_at(split_point);
        let (_, mask_str) = s.split_at(split_point + 1);
        let network_address = IpAddr::from_str(address_str)?;
        let cidr = u8::from_str(mask_str)?;
        let max_cidr = match network_address {
            IpAddr::V4(_) => 32,
            IpAddr::V6(_) => 128,
        };
        if cidr > max_cidr {
            return Err(IpAddrRangeError::ParseError);
        }
        let range = match network_address {
            IpAddr::V4(ipv4) => IpAddrRange::V4(IpAddrRangeV4::new(ipv4, cidr)),
            IpAddr::V6(ipv6) => IpAddrRange::V6(IpAddrRangeV6::new(ipv6, cidr)),
        };
        Ok(range)
    }
}

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

    #[test]
    fn from_str_valid_v4() {
        let from_str = IpAddrRange::from_str("127.0.0.1/24").unwrap();
        let from_ints = IpAddrRange::V4(IpAddrRangeV4::new(Ipv4Addr::new(127, 0, 0, 1), 24));

        assert_eq!(from_str, from_ints);
    }

    #[test]
    fn from_str_valid_v6() {
        let from_str = IpAddrRange::from_str("::1/24").unwrap();
        let from_ints = IpAddrRange::V6(IpAddrRangeV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1),
                                                           24));

        assert_eq!(from_str, from_ints);
    }

    #[test]
    fn from_str_invalid() {
        let from_str = IpAddrRange::from_str("not_and_ip/not_a_cidr");
        assert!(from_str.is_err());
    }
}
