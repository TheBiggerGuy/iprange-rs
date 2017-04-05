use std::fmt;
use std::net::Ipv4Addr;
use std::result::Result::{self, Ok, Err};
use std::str::FromStr;

use iprange::IpAddrRangeError;
use bits::{ipv4_to_u32, number_of_common_prefix_bits_u32, prefix_mask_u32};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IpAddrRangeV4 {
    network_address: Ipv4Addr,
    cidr: u8,
}

impl IpAddrRangeV4 {
    /// Constructs a new `IpAddrRangeV4` using a `Ipv4Addr` and CIDR prefix.
    pub fn new(network_address: Ipv4Addr, cidr: u8) -> IpAddrRangeV4 {
        assert!(cidr <= 32);
        IpAddrRangeV4 {
            network_address: network_address,
            cidr: cidr,
        }
    }

    // TODO:
    // * rfc3021
    /// Constructs a new `IpAddrRangeV4` using a `Ipv4Addr` network and broadcast address.
    pub fn from_range(network_address: Ipv4Addr,
                      broadcast_address: Ipv4Addr)
                      -> Result<IpAddrRangeV4, IpAddrRangeError> {
        if network_address == broadcast_address {
            return Ok(IpAddrRangeV4 {
                          network_address: network_address,
                          cidr: 32,
                      });
        }
        let network = ipv4_to_u32(&network_address);
        let broadcast = ipv4_to_u32(&broadcast_address);
        let cidr = number_of_common_prefix_bits_u32(network, broadcast);

        let net_mask = prefix_mask_u32(cidr);
        let host_mask = !net_mask;

        if (network & host_mask) != 0 {
            return Err(IpAddrRangeError::InvalidNetworkAddress);
        }
        if broadcast != (network | host_mask) {
            return Err(IpAddrRangeError::InvalidNetworkAddress);
        }

        Ok(IpAddrRangeV4 {
               network_address: network_address,
               cidr: cidr,
           })
    }

    pub fn network_address(&self) -> Ipv4Addr {
        self.network_address
    }

    pub fn cidr(&self) -> u8 {
        self.cidr
    }
}

impl fmt::Display for IpAddrRangeV4 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}/{}", self.network_address, self.cidr)
    }
}

impl FromStr for IpAddrRangeV4 {
    type Err = IpAddrRangeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split_point = s.find('/').ok_or(IpAddrRangeError::ParseError)?;
        let (address_str, _) = s.split_at(split_point);
        let (_, mask_str) = s.split_at(split_point + 1);
        let network_address = Ipv4Addr::from_str(address_str)?;
        let cidr = u8::from_str(mask_str)?;
        if cidr > 32 {
            return Err(IpAddrRangeError::ParseError);
        }
        Ok(IpAddrRangeV4::new(network_address, cidr))
    }
}

#[cfg(test)]
mod tests {
    extern crate env_logger;

    use std::net::{IpAddr, Ipv4Addr};
    use std::str::FromStr;

    use iprange::IpAddrRange;

    use super::*;

    fn ipv4(s: &str) -> Ipv4Addr {
        Ipv4Addr::from_str(s).unwrap()
    }

    #[test]
    fn from_range_error_start_after_end() {
        let ip1 = ipv4("127.0.0.2");
        let ip2 = ipv4("127.0.0.1");
        let result = IpAddrRange::from_range(IpAddr::V4(ip1), IpAddr::V4(ip2));
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error, IpAddrRangeError::InvalidNetworkAddress);
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
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn from_range_simple_netmask() {
        let _ = env_logger::init();

        let ip1 = ipv4("127.0.0.0");
        let ip2 = ipv4("127.0.0.255");
        let result = IpAddrRange::from_range(IpAddr::V4(ip1), IpAddr::V4(ip2));
        assert!(result.is_ok());
        let range = result.unwrap();
        match range {
            IpAddrRange::V4(range_v4) => {
                assert_eq!(range_v4.to_string(), String::from("127.0.0.0/24"));
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn to_string() {
        let range = IpAddrRangeV4::new(Ipv4Addr::from_str("127.0.0.1").unwrap(), 24);

        assert_eq!(range.to_string(), "127.0.0.1/24");
    }

    #[test]
    fn from_str_valid() {
        let from_str = IpAddrRangeV4::from_str("127.0.0.1/24").unwrap();
        let from_ints = IpAddrRangeV4::new(Ipv4Addr::new(127, 0, 0, 1), 24);

        assert_eq!(from_str, from_ints);
    }

    #[test]
    fn from_str_invalid_trailing_slash() {
        let from_str = IpAddrRangeV4::from_str("127.0.0.1/");
        assert!(from_str.is_err());
    }

    #[test]
    fn from_str_invalid_leading_slash() {
        let from_str = IpAddrRangeV4::from_str("/24");
        assert!(from_str.is_err());
    }

    #[test]
    fn from_str_invalid_missing_cidr() {
        let from_str = IpAddrRangeV4::from_str("127.0.0.1");
        assert!(from_str.is_err());
    }

    #[test]
    fn from_str_invalid_missing_ip() {
        let from_str = IpAddrRangeV4::from_str("24");
        assert!(from_str.is_err());
    }

    #[test]
    fn from_str_invalid_multiple_slashes() {
        let from_str = IpAddrRangeV4::from_str("127.0.0.1/24/");
        assert!(from_str.is_err());
    }

    #[test]
    fn from_str_invalid_ip() {
        let from_str = IpAddrRangeV4::from_str("256.0.0.1/24");
        assert!(from_str.is_err());
    }

    #[test]
    fn from_str_invalid_cidr() {
        let from_str = IpAddrRangeV4::from_str("127.0.0.1/33");
        assert!(from_str.is_err());
    }

    #[test]
    fn from_str_invalid_empty_str() {
        let from_str = IpAddrRangeV4::from_str("");
        assert!(from_str.is_err());
    }
}
