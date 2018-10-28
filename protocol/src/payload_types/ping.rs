//The ping message is sent primarily to confirm that the TCP/IP connection is still valid.
// An error in transmission is presumed to be a closed connection and the address is removed as a current peer.

use super::payload::Payload;
use serialization::stream::Stream;
use serialization::reader::Error;

pub struct Ping{
    pub nonce: u64,
}

impl Payload for Ping {
    fn command() -> &'static str {
        "ping"
    }

    fn serialize(&self, stream: & mut Stream) -> Result<(), Error> {
        unimplemented!()
    }
}