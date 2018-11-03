// see https://bitcoin.org/en/developer-reference#block-headers

use basictype::hash;
use serialization::reader::Deserializable;
use serialization::reader::Reader;

// see https://en.bitcoin.it/wiki/Protocol_documentation#Block_Headers
#[derive(Monad_Serializable, Monad_Deserializable, Debug)]
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
    use serialization::stream::{Serializable, Stream};
    use super::BlockHeader;
    use super::Deserializable;
    use super::Reader;

    #[test]
    fn test() {
        let hh = hash::Hash256::from_reversed_string("0000000000000000000383fb0c96397da185a378d04cf7d451ef81a7b446fbb7");
        let mut stream = Stream::new();
        hh.serialize(&mut stream);
    }

    #[test]
    fn test_deserialize_with_annotation() {
        let buf = [1u8, 0, 0, 0, 183, 251, 70, 180, 167, 129, 239, 81, 212, 247, 76, 208, 120, 163, 133, 161, 125, 57, 150, 12, 251, 131, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 183, 251, 70, 180, 167, 129, 239, 81, 212, 247, 76, 208, 120, 163, 133, 161, 125, 57, 150, 12, 251, 131, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let mut reader = Reader::from_bytes(&buf);
        let result = BlockHeader::deserialize(&mut reader);
        println!("{:?}", result)
    }

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

        println!("{:?}", stream);
    }

    #[test]
    fn test_serialize_size_with_annotation() {
        let block_header = BlockHeader {
            version: 1,
            previous_block_header_hash: hash::Hash256::from_reversed_string("0000000000000000000383fb0c96397da185a378d04cf7d451ef81a7b446fbb7"),
            merkle_root_hash: hash::Hash256::from_reversed_string("0000000000000000000383fb0c96397da185a378d04cf7d451ef81a7b446fbb7"),
            time: 0,
            n_bits: 0,
            nonce: 0,
        };

        let size = block_header.serialized_size();

        println!("size is {}", size);
    }
}