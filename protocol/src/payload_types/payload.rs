use serialization::stream::Stream;
use std::io::Error;

pub trait Payload {
    fn command() -> &'static str;

//    fn serialize(stream: &mut Stream) -> Result<(), Error>;
}