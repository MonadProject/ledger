//The getaddr message sends a request to a node asking for information about known active peers to
// help with finding potential nodes in the network. The response to receiving this message is
// to transmit one or more addr messages with one or more peers from a database of known active peers.
// The typical presumption is that a node is likely to be active if it has been sending a message within
// the last three hours.


use payload::Payload;
use serialization::stream::Stream;
use serialization::reader::Error;


//No additional data is transmitted with this message.
pub struct GetAddr;

impl Payload for GetAddr {
    fn command() -> &'static str {
        "getaddr"
    }

    fn serialize(&self, stream: & mut Stream) -> Result<(), Error> {
        unimplemented!()
    }
}