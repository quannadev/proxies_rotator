use bstr::ByteSlice;
use std::fmt;

#[derive(Debug)]
pub enum SocksAddrType<'a> {
    Ipv4(&'a [u8]),
    Domain(&'a [u8]),
    Ipv6(&'a [u8]),
}

impl<'a> fmt::Display for SocksAddrType<'a> {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SocksAddrType::Ipv4(b) => write!(w, "{}.{}.{}.{}", b[0], b[1], b[2], b[3]),
            SocksAddrType::Domain(b) => write!(w, "{:?}", b.as_bstr()),
            SocksAddrType::Ipv6(b) => write!(w, "[{:02X?}{:02X?}:{:02X?}{:02X?}:{:02X?}{:02X?}:{:02X?}{:02X?}:{:02X?}{:02X?}:{:02X?}{:02X?}:{:02X?}{:02X?}:{:02X?}{:02X?}]",
                                             b[0], b[1], b[2], b[3],
                                             b[4], b[5], b[6], b[7],
                                             b[8], b[9], b[10], b[11],
                                             b[12], b[13], b[14], b[15],
            ),
        }
    }
}

impl<'a> SocksAddrType<'a> {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![5, 1, 0];
        match self {
            SocksAddrType::Ipv4(addr) => {
                buf.push(1); // Address type (IPv4)
                buf.extend_from_slice(addr); // Address bytes
            }
            SocksAddrType::Domain(domain) => {
                buf.push(3); // Address type (domain name)
                buf.push(domain.len() as u8); // Domain name length
                buf.extend_from_slice(domain); // Domain name bytes
            }
            SocksAddrType::Ipv6(addr) => {
                buf.push(4); // Address type (IPv6)
                buf.extend_from_slice(addr); // Address bytes
            }
        }
        buf
    }
}
