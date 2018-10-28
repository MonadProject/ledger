//The headers packet returns block headers in response to a getheaders packet.

use blockchain::block_header::BlockHeader;
use super::payload::Payload;
use serialization::stream::Stream;
use serialization::reader::Error;


pub struct Headers(Vec<BlockHeader>);

impl Payload for Headers {
    fn command() -> &'static str {
        "headers"
    }

    fn serialize(&self, stream: & mut Stream) -> Result<(), Error> {
        unimplemented!()
    }
}