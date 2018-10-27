use common::network_address::Network_Address;

//see https://en.bitcoin.it/wiki/Protocol_documentation#addr


//Provide information on known nodes of the network.
// Non-advertised nodes should be forgotten after typically 3 hours
pub struct Addr([AddressEntry]);

pub struct AddressEntry {
    timestamp: u32,
    net_addr: Network_Address,
}
