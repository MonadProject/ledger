use payload::Payload;
use serialization::stream::Stream;
use serialization::reader::Error;

pub struct FeeFilter {
    pub feerate: u64,
}

impl Payload for FeeFilter {
    fn command() -> &'static str {
        "feefilter"
    }

    fn serialize(&self, stream: & mut Stream) -> Result<(), Error> {
        unimplemented!()
    }
}

