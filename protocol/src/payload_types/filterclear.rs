use super::payload::Payload;
use serialization::stream::Stream;
use serialization::reader::Error;

pub struct FilterClear;

impl Payload for FilterClear{
    fn command() -> &'static str {
        "filterclear"
    }

    fn serialize(&self, stream: & mut Stream) -> Result<(), Error> {
        unimplemented!()
    }
}
