//notfound is a response to a getdata, sent if any requested data items could not be relayed, for example,
// because the requested transaction was not in the memory pool or relay set.

use common::inventory_vectors::InventoryVectors;
use super::payload::Payload;


pub struct NotFound(Vec<InventoryVectors>);

impl Payload for NotFound {
    fn command() -> &'static str {
        "notfound"
    }
}