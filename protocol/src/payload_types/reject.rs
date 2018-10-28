//The reject message is sent when messages are rejected.
use super::payload::Payload;
use serialization::stream::Stream;
use serialization::reader::Error;

pub struct Reject {
    pub message: String,
    pub ccode: u8,
    pub reason: String,
    pub data: u8,

}

impl Payload for Reject {
    fn command() -> &'static str {
        "reject"
    }

    fn serialize(&self, stream: & mut Stream) -> Result<(), Error> {
        unimplemented!()
    }
}
