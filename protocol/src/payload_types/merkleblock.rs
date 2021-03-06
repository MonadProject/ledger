use basictype::bytes::Bytes;
use basictype::hash::Hash256;
use payload::Payload;
use serialization::stream::Stream;
use serialization::reader::Error;


pub struct MerkleBlock {
    pub version: u32,
    pub prev_block: Hash256,
    pub merkle_root: Hash256,
    pub timestamp: u32,
    pub bits: u32,
    pub nonce: u32,
    pub total_transactions: u32,
    pub hashes: Vec<Hash256>,
    pub flags: Bytes,
}

impl Payload for MerkleBlock {
    fn command() -> &'static str {
        "merkleblock"
    }

    fn serialize(&self, stream: & mut Stream) -> Result<(), Error> {
        unimplemented!()
    }
}