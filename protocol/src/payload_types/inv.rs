use common::inventory_vectors::InventoryVectors;
//see https://en.bitcoin.it/wiki/Protocol_documentation#inv

//Allows a node to advertise its knowledge of one or more objects. It can be received unsolicited,
// or in reply to getblocks.

//Payload (maximum 50,000 entries, which is just over 1.8 megabytes):

use super::payload::Payload;


pub struct Inventory(Vec<InventoryVectors>);

impl Payload for Inventory {
    fn command() -> &'static str {
        "inv"
    }
}