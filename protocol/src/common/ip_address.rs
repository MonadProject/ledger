use serialization::reader::{Deserializable, Reader};
use serialization::reader::Error;
use serialization::stream::{Serializable, Stream};
use std::io;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;

#[derive(Debug)]
pub struct IpAddress(IpAddr);

impl IpAddress {
    pub fn from_ip_addr(ip_addr: IpAddr) -> Self {
        IpAddress(ip_addr)
    }
}

pub fn deserialize_u16_from_slice(buffer: &[u8]) -> u16 {
    let mut reader = Reader::from_bytes(&buffer);
    reader.read().unwrap()
}


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

impl Deserializable for IpAddress {
    fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, Error> where Self: Sized, T: io::Read {
        let mut buffer = [0u8; 16];
        reader.read_exact(&mut buffer);
        match buffer[0..12] {
            [0u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff] => Ok(IpAddress(IpAddr::V4(Ipv4Addr::new(buffer[12], buffer[13], buffer[14], buffer[15])))),

            _ => Ok(IpAddress(IpAddr::V6(Ipv6Addr::new(deserialize_u16_from_slice(&buffer[0..2]), deserialize_u16_from_slice(&buffer[2..4]), deserialize_u16_from_slice(&buffer[4..6]), deserialize_u16_from_slice(&buffer[6..8]),
                                                       deserialize_u16_from_slice(&buffer[8..10]), deserialize_u16_from_slice(&buffer[10..12]), deserialize_u16_from_slice(&buffer[12..14]), deserialize_u16_from_slice(&buffer[14..16]))))),
        }
    }
}

impl IpAddress {}


#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;
    use std::net::Ipv6Addr;
    use super::IpAddr;
    use super::IpAddress;
    use super::Serializable;
    use super::Stream;
    use super::Reader;
    use super::Deserializable;

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

    #[test]
    fn test_deserializable_for_ipv4_address() {
        let buffer = [0u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 127, 0, 0, 1];
        let mut reader =  Reader::from_bytes(&buffer);
        let ip_address = IpAddress::deserialize(&mut reader);
        println!("{:?}",ip_address);
    }

    //todo try to verify this test
    #[test]
    fn test_deserializable_for_ipv6_address() {
        let buffer = [0u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
        let mut reader =  Reader::from_bytes(&buffer);
        let ip_address = IpAddress::deserialize(&mut reader);
        println!("{:?}",ip_address);
    }

}

