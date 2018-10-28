//Request for Direct headers announcement.
//
//Upon receipt of this message, the node is be permitted, but not required, to announce new blocks by headers command (instead of inv command).
//
//This message is supported by the protocol version >= 70012 or Bitcoin Core version >= 0.12.0.
//
//See BIP 130 for more information.
//
//No additional data is transmitted with this message.

use super::payload::Payload;


pub struct SendHeaders;

impl Payload for SendHeaders {
    fn command() -> &'static str {
        "sendheaders"
    }
}