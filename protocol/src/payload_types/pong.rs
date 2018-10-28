//The pong message is sent in response to a ping message.
// In modern protocol versions, a pong response is generated using a nonce included in the ping.
use super::payload::Payload;
use serialization::stream::Stream;
use serialization::reader::Error;


pub struct Pong {
    pub nonce: u64,
}

impl Payload for Pong {
    fn command() -> &'static str {
        "pong"
    }

    fn serialize(&self, stream: & mut Stream) -> Result<(), Error> {
        unimplemented!()
    }
}