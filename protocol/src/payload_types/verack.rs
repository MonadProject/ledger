//The verack message is sent in reply to version.
// This message consists of only a message header with the command string "verack".
//  no body

use super::payload::Payload;

pub struct VersionAck;

impl Payload for VersionAck {
    fn command() -> &'static str {
        "verack"
    }
}