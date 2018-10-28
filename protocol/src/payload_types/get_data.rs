// see https://en.bitcoin.it/wiki/Protocol_documentation#getdata


//getdata is used in response to inv, to retrieve the content of a specific object,
// and is usually sent after receiving an inv packet, after filtering known elements.
// It can be used to retrieve transactions, but only if they are in the memory pool or
// relay set - arbitrary access to transactions in the chain is not allowed to avoid
// having clients start to depend on nodes having full transaction indexes (which modern nodes do not).

use common::inventory_vectors::InventoryVectors;
use super::payload::Payload;
use serialization::stream::Stream;
use serialization::reader::Error;

pub struct GetData(Vec<InventoryVectors>);

impl Payload for GetData {
    fn command() -> &'static str {
        "getdata"
    }

    fn serialize(&self, stream: & mut Stream) -> Result<(), Error> {
        unimplemented!()
    }
}