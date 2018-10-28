use common::network_address::Network_Address;
use super::payload::Payload;
use serialization::reader::Error;
use serialization::stream::Stream;
use serialization::compact::Compact;

//see https://en.bitcoin.it/wiki/Protocol_documentation#addr


//Provide information on known nodes of the network.
// Non-advertised nodes should be forgotten after typically 3 hours
pub struct Addr([AddressEntry]);

pub struct AddressEntry {
    timestamp: u32,
    net_addr: Network_Address,
}


impl Payload for Addr {
    fn command() -> &'static  str {
        "addr"
    }

    fn serialize(&self, stream: & mut Stream) -> Result<(), Error> {
        unimplemented!()
    }
}
