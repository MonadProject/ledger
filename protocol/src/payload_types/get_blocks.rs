//https://en.bitcoin.it/wiki/Protocol_documentation#getdata

//Return an inv packet containing the list of blocks starting right after the last known hash in the block locator object,
// up to hash_stop or 500 blocks, whichever comes first.
//The locator hashes are processed by a node in the order as they appear in the message.
// If a block hash is found in the node's main chain, the list of its children is returned back
// via the inv message and the remaining locators are ignored,
// no matter if the requested limit was reached, or not.
//To receive the next blocks hashes, one needs to issue getblocks again with a new block locator object.
// Keep in mind that some clients may provide blocks which are invalid if the block locator object contains a hash on the invalid branch.

use basictype::hash::Hash256;
use payload_types::version::Version;
use payload::Payload;
use serialization::stream::Stream;
use serialization::reader::Error;

pub struct GetBlocks {
    //the protocol version
    pub version: Version,
    //block locator object; newest back to genesis block (dense to start, but then sparse)
    pub block_locator_hashes: Hash256,
    //hash of the last desired block; set to zero to get as many blocks as possible (500)
    pub hash_stop: Hash256,

}

impl Payload for GetBlocks {
    fn command() -> &'static str {
        "getblocks"
    }

    fn serialize(&self, stream: & mut Stream) -> Result<(), Error> {
        unimplemented!()
    }
}