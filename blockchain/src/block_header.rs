// see https://bitcoin.org/en/developer-reference#block-headers

use basictype::hash;

pub struct BlockHeader {
    pub version: u32,
    pub previous_block_header_hash: hash::Hash256,
    pub merkle_root_hash: hash::Hash256,
    pub time: u32,
    pub nBits: u32,
    pub nonce: u32,
}
