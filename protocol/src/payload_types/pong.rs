//The pong message is sent in response to a ping message.
// In modern protocol versions, a pong response is generated using a nonce included in the ping.

pub struct Pong {
    pub nonce: u64,
}