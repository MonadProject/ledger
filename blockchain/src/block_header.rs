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


#[cfg(test)] //cargo test -- --nocapture
mod tests {
    use basictype::hash;
    use super::BlockHeader;
    use serialization::stream::{Serializable,Stream};

    #[test]
    fn test_serialize_with_annotation() {

        let block_header = BlockHeader {
            version: 1,
            previous_block_header_hash: hash::Hash256::from_reversed_string("0000000000000000000383fb0c96397da185a378d04cf7d451ef81a7b446fbb7"),
            merkle_root_hash: hash::Hash256::from_reversed_string("0000000000000000000383fb0c96397da185a378d04cf7d451ef81a7b446fbb7"),
            time: 0,
            n_bits: 0,
            nonce: 0,
        };

        let mut stream = Stream::new();

        block_header.serialize(&mut stream);

        println!("{:?}",stream);
    }

    #[test]
    fn test_serialize_size_with_annotation() {

    }
}