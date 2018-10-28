//The verack message is sent in reply to version.
// This message consists of only a message header with the command string "verack".
//  no body

use super::payload::Payload;
use serialization::stream::Stream;
use serialization::reader::Error;

pub struct VersionAck;

impl Payload for VersionAck {
    fn command() -> &'static str {
        "verack"
    }

    fn serialize(&self, stream: &mut Stream) -> Result<(), Error> {
        unimplemented!()
    }
}