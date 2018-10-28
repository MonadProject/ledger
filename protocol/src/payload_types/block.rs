//The block message is sent in response to a getdata message
// which requests transaction information from a block hash.
use blockchain::block::MonadBlock;
use super::payload::Payload;
use serialization::stream::Stream;
use serialization::reader::Error;


pub struct Block(MonadBlock);

impl Payload for Block {
    fn command() -> &'static str {
        "block"
    }

    fn serialize(&self, stream: & mut Stream) -> Result<(), Error> {
        unimplemented!()
    }
}

