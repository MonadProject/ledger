use serialization::reader::Error;
use serialization::stream::Stream;

pub trait Payload {
    fn command() -> &'static str;

    fn serialize(&self, stream: &mut Stream) -> Result<(), Error>;
}