use common::ip_address::IpAddress;
use common::services::Services;
use serialization::stream::{Serializable, Stream};

// see https://en.bitcoin.it/wiki/Protocol_documentation#Network_address
#[derive(Debug)]
pub struct Network_Address {
    services: Services,
    ipAddress: IpAddress,
    port: u16,
}

impl Serializable for Network_Address {
    fn serialize(&self, s: &mut Stream) {
        let mut services_stream = Stream::new();
        self.services.serialize(&mut services_stream);

        s.write_slice(&services_stream.take());

        let mut ip_address_stream = Stream::new();
        self.ipAddress.serialize(&mut ip_address_stream);

        s.write_slice(&ip_address_stream.take());
        s.write(&self.port);
    }

    fn serialized_size(&self) -> usize {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use std::net::IpAddr;
    use std::net::Ipv4Addr;
    use super::IpAddress;
    use super::Network_Address;
    use super::Serializable;
    use super::Services;
    use super::Stream;

    #[test]
    fn test_serialize() {
        let network_address = Network_Address {
            services: Services::from_u64(1u64),
            ipAddress: IpAddress::from_ip_addr(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
            port: 25535u16,
        };
        let mut stream = Stream::new();

        network_address.serialize(&mut stream);

        println!("{:?}", stream);
    }
}