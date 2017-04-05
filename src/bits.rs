extern crate test;

use std::net::{Ipv4Addr, Ipv6Addr};

pub fn ipv4_to_u32(ip: &Ipv4Addr) -> u32 {
    ip.octets()
        .iter()
        .rev()
        .enumerate()
        .fold(0u32,
              |acc, (count, bits)| acc | ((*bits as u32) << (count * 8)))
}

pub fn ipv6_to_u128(ip: &Ipv6Addr) -> u128 {
    ip.octets()
        .iter()
        .rev()
        .enumerate()
        .fold(0u128,
              |acc, (count, bits)| acc | ((*bits as u128) << (count * 8)))
}

#[inline]
pub fn number_of_common_prefix_bits_u32(a: u32, b: u32) -> u8 {
    (a ^ b).leading_zeros() as u8
}

#[inline]
pub fn number_of_common_prefix_bits_u128(a: u128, b: u128) -> u8 {
    (a ^ b).leading_zeros() as u8
}

pub fn prefix_mask_u32(leading_zeros: u8) -> u32 {
    assert!(leading_zeros <= 32);
    let mut mask: u32 = 0;
    for i in (32 - leading_zeros)..32 {
        mask |= 1 << i;
    }
    mask
}

pub fn prefix_mask_u128(leading_zeros: u8) -> u128 {
    assert!(leading_zeros <= 128);
    let mut mask: u128 = 0;
    for i in (128 - leading_zeros)..128 {
        mask |= 1 << i;
    }
    mask
}

#[cfg(test)]
mod tests {
    extern crate env_logger;

    use std::net::{Ipv4Addr, Ipv6Addr};
    use std::str::FromStr;

    use test::Bencher;

    use super::*;

    fn ipv4(s: &str) -> Ipv4Addr {
        Ipv4Addr::from_str(s).unwrap()
    }

    fn ipv6(s: &str) -> Ipv6Addr {
        Ipv6Addr::from_str(s).unwrap()
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
    fn ipv6_to_u128_zero() {
        let ip = ipv6("::");
        assert_eq!(ipv6_to_u128(&ip), 0x0000_0000_0000_0000_0000_0000_0000_0000);
    }

    #[test]
    fn ipv6_to_u128_le_be() {
        let ip1 = ipv6("::ffff:ffff:ffff:ffff");
        assert_eq!(ipv6_to_u128(&ip1),
                   0x0000_0000_0000_0000_ffff_ffff_ffff_ffffu128);
        let ip2 = ipv6("ffff:ffff:ffff:ffff::");
        assert_eq!(ipv6_to_u128(&ip2), 0xffff_ffff_ffff_ffff_0000_0000_0000_0000u128);
    }

    #[test]
    fn ipv6_to_u128_ff() {
        let ip = ipv6("ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff");
        assert_eq!(ipv6_to_u128(&ip), 0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff);
    }

    #[test]
    fn ipv6_to_u128_localhost() {
        let ip = ipv6("::1");
        assert_eq!(ipv6_to_u128(&ip), 0x0000_0000_0000_0000_0000_0000_0000_0001);
    }

    #[test]
    fn prefix_mask_u32_test() {
        assert_eq!(prefix_mask_u32(0),  0b00000000_00000000_00000000_00000000);
        assert_eq!(prefix_mask_u32(1),  0b10000000_00000000_00000000_00000000);
        assert_eq!(prefix_mask_u32(16), 0b11111111_11111111_00000000_00000000);
        assert_eq!(prefix_mask_u32(31), 0b11111111_11111111_11111111_11111110);
        assert_eq!(prefix_mask_u32(32), 0b11111111_11111111_11111111_11111111);
    }

    #[bench]
    fn bench_ipv4_to_u32(b: &mut Bencher) {
        let ip = ipv4("127.0.0.1");
        b.iter(|| ipv4_to_u32(&ip));
    }

    #[bench]
    fn bench_ipv6_to_u128(b: &mut Bencher) {
        let ip = ipv6("2001::1");
        b.iter(|| ipv6_to_u128(&ip));
    }
}
