//The reject message is sent when messages are rejected.
use super::payload::Payload;

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
}
