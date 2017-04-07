use std::fmt;
use std::net::Ipv6Addr;
use std::result::Result::{self, Ok, Err};
use std::str::FromStr;

use iprange::IpAddrRangeError;
use bits::{ipv6_to_u128, number_of_common_prefix_bits_u128, prefix_mask_u128};

/// Representation of an IPv4 address range.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct IpAddrRangeV6 {
    network_address: Ipv6Addr,
    cidr: u8,
}

impl IpAddrRangeV6 {
    /// Constructs a new `IpAddrRangeV6` using a `Ipv6Addr` and CIDR prefix.
    pub fn new(network_address: Ipv6Addr, cidr: u8) -> IpAddrRangeV6 {
        assert!(cidr <= 128);
        IpAddrRangeV6 {
            network_address: network_address,
            cidr: cidr,
        }
    }

    // TODO:
    /// Constructs a new `IpAddrRangeV6` using a `Ipv6Addr` network and broadcast address.
    pub fn from_range(network_address: Ipv6Addr,
                      broadcast_address: Ipv6Addr)
                      -> Result<IpAddrRangeV6, IpAddrRangeError> {
        if network_address == broadcast_address {
            return Ok(IpAddrRangeV6 {
                          network_address: network_address,
                          cidr: 128,
                      });
        }
        let network = ipv6_to_u128(&network_address);
        let broadcast = ipv6_to_u128(&broadcast_address);
        let cidr = number_of_common_prefix_bits_u128(network, broadcast);

        let net_mask = prefix_mask_u128(cidr);
        let host_mask = !net_mask;

        if (network & host_mask) != 0 {
            return Err(IpAddrRangeError::InvalidNetworkAddress);
        }
        if broadcast != (network | host_mask) {
            return Err(IpAddrRangeError::InvalidNetworkAddress);
        }

        Ok(IpAddrRangeV6 {
               network_address: network_address,
               cidr: cidr,
           })
    }

    pub fn network_address(&self) -> Ipv6Addr {
        self.network_address
    }

    pub fn cidr(&self) -> u8 {
        self.cidr
    }
}

impl fmt::Display for IpAddrRangeV6 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}/{}", self.network_address, self.cidr)
    }
}

impl FromStr for IpAddrRangeV6 {
    type Err = IpAddrRangeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split_point = s.rfind('/').ok_or(IpAddrRangeError::IpAddrRangeParseError)?;
        let address_str = &s[..split_point];
        let mask_str = &s[split_point + 1..];
        if address_str.len() == 0 || mask_str.len() == 0 {
            return Err(IpAddrRangeError::IpAddrRangeParseError);
        }

        let network_address = Ipv6Addr::from_str(address_str)?;
        let cidr = u8::from_str(mask_str)?;
        if cidr > 128 {
            return Err(IpAddrRangeError::InvalidCidr(cidr));
        }

        Ok(IpAddrRangeV6::new(network_address, cidr))
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv6Addr};
    use std::str::FromStr;

    use test::Bencher;

    use iprange::IpAddrRange;

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
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn from_range_simple_netmask() {
        let ip1 = Ipv6Addr::from_str("::0").unwrap();
        let ip2 = Ipv6Addr::from_str("::ffff").unwrap();
        let result = IpAddrRange::from_range(IpAddr::V6(ip1), IpAddr::V6(ip2));
        assert!(result.is_ok());
        let range = result.unwrap();
        match range {
            IpAddrRange::V6(range_v6) => {
                assert_eq!(range_v6.to_string(), String::from("::/112"));
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn to_string() {
        let range = IpAddrRangeV6::new(Ipv6Addr::from_str("::1").unwrap(), 24);

        assert_eq!(range.to_string(), "::1/24");
    }

    #[test]
    fn from_str_valid() {
        let from_str = IpAddrRangeV6::from_str("::1/24").unwrap();
        let from_ints = IpAddrRangeV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 24);

        assert_eq!(from_str, from_ints);
    }

    #[test]
    fn from_str_invalid_trailing_slash() {
        let from_str = IpAddrRangeV6::from_str("::1/");
        assert!(from_str.is_err());
    }

    #[test]
    fn from_str_invalid_leading_slash() {
        let from_str = IpAddrRangeV6::from_str("/24");
        assert!(from_str.is_err());
    }

    #[test]
    fn from_str_invalid_missing_cidr() {
        let from_str = IpAddrRangeV6::from_str("::1");
        assert!(from_str.is_err());
    }

    #[test]
    fn from_str_invalid_missing_ip() {
        let from_str = IpAddrRangeV6::from_str("24");
        assert!(from_str.is_err());
    }

    #[test]
    fn from_str_invalid_multiple_slashes() {
        let from_str = IpAddrRangeV6::from_str("::1/24/");
        assert!(from_str.is_err());
    }

    #[test]
    fn from_str_invalid_ip() {
        let from_str = IpAddrRangeV6::from_str("abs/24");
        assert!(from_str.is_err());
    }

    #[test]
    fn from_str_invalid_cidr() {
        let from_str = IpAddrRangeV6::from_str("::1/129");
        assert!(from_str.is_err());
    }

    #[test]
    fn from_str_invalid_empty_str() {
        let from_str = IpAddrRangeV6::from_str("");
        assert!(from_str.is_err());
    }

    #[bench]
    fn bench_from_str(b: &mut Bencher) {
        b.iter(|| IpAddrRangeV6::from_str("2001::1/24"));
    }
}
