use serialization::stream::{Serializable, Stream};
use std::net::IpAddr;

pub struct IpAddress(IpAddr);


//Pv6 address. Network byte order. The original client only supported IPv4 and only read the last 4 bytes to get the IPv4 address.
// However, the IPv4 address is written into the message as a 16 byte IPv4-mapped IPv6 address
//(12 bytes 00 00 00 00 00 00 00 00 00 00 FF FF, followed by the 4 bytes of the IPv4 address).
impl Serializable for IpAddress {
    fn serialize(&self, s: &mut Stream) {
        match self.0 {
            IpAddr::V4(address) => {
                s.write_slice(&[0u8; 10]);
                s.write_slice(&[0xffu8, 0xffu8]);
                s.write_slice(&address.octets());
            }
            IpAddr::V6(address) => {
                s.write_slice(&address.octets());
            }
        }
    }

    fn serialized_size(&self) -> usize {
        match self.0 {
            IpAddr::V4(address) => {
                16
            }
            IpAddr::V6(address) => {
                16
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;
    use std::net::Ipv6Addr;
    use super::IpAddr;
    use super::IpAddress;
    use super::Serializable;
    use super::Stream;

    #[test]
    fn test_serialize_ipv4() {
        let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let ipAddress = IpAddress(localhost_v4);
        let mut stream = Stream::new();
        ipAddress.serialize(&mut stream);
        println!("{:?}", stream);
    }

    #[test]
    fn test_serialize_ipv6() {
        let localhost_v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
        let ipAddress = IpAddress(localhost_v6);
        let mut stream = Stream::new();
        ipAddress.serialize(&mut stream);
        println!("{:?}", stream);
    }
}

