use basictype::hash::Hash32;
use common::command::Command;
//see https://bitcoin.org/en/developer-reference#message-headers
use network::network::Magic;

pub struct MessageHeader {
    magic: Magic,
    command: Command,
    payload_size: u32,
    checksum: Hash32,
}