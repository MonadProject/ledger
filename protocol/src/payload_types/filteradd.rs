use basictype::bytes::Bytes;
use payload::Payload;
use serialization::stream::Stream;
use serialization::reader::Error;


pub struct FilterAdd {
    pub data: Bytes,
}

impl Payload for FilterAdd {
    fn command() -> &'static str {
        "filteradd"
    }

    fn serialize(&self, stream: & mut Stream) -> Result<(), Error> {
        unimplemented!()
    }
}