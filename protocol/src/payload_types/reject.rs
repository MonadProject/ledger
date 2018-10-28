//The reject message is sent when messages are rejected.

pub struct Reject {
    pub message: String,
    pub ccode: u8,
    pub reason: String,
    pub data: u8,

}
