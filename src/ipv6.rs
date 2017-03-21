use std::fmt;
use std::net::Ipv6Addr;
use std::result::Result;
use std::result::Result::Ok;

use iprange::{IpAddrRange, IpAddrRangeError};
use bits::{ipv6_to_u128, number_of_common_prefix_bits_u128, prefix_mask_u128};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IpAddrRangeV6 {
    network_address: Ipv6Addr,
    cidr: u8,
}

impl IpAddrRangeV6 {
    /// Constructs a new `IpAddrRangeV4` using a `Ipv4Addr` and CIDR prefix.
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

/*
impl ToString for IpAddrRangeV6 {
    fn to_string(&self) -> String {
        format!("{}/{}", self.network_address, self.cidr())
    }
}
*/

impl fmt::Display for IpAddrRangeV6 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}/{}", self.network_address, self.cidr)
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv6Addr};
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
}
