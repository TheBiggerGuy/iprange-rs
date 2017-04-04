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

impl fmt::Display for IpAddrRange {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IpAddrRange::V4(ref r) => r.fmt(fmt),
            IpAddrRange::V6(ref r) => r.fmt(fmt),
        }
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
}
