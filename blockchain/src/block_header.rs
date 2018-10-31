// see https://bitcoin.org/en/developer-reference#block-headers

use basictype::hash;

// see https://en.bitcoin.it/wiki/Protocol_documentation#Block_Headers
#[derive(Monad_Serializable)]
pub struct BlockHeader {
    pub version: u32,
    pub previous_block_header_hash: hash::Hash256,
    pub merkle_root_hash: hash::Hash256,
    pub time: u32,
    pub n_bits: u32,
    pub nonce: u32,
}