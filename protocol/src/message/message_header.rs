use basictype::hash::Hash32;
use common::command::Command;
use network::network::Magic;
use serialization::reader::Deserializable;
use serialization::stream::{Serializable, Stream};
use std::io::Error;


//see https://bitcoin.org/en/developer-reference#message-headers
pub struct MessageHeader {
    magic: Magic,
    command: Command,
    payload_size: u32,
    checksum: Hash32,
}

pub type MessageResult<T> = Result<T, Error>;

impl Serializable for MessageHeader {
    fn serialize(&self, s: &mut Stream) {
        s.write(&self.magic);
        s.write(&self.command);
        s.write(&self.payload_size);
        s.write(&self.checksum);
    }

    fn serialized_size(&self) -> usize {
        unimplemented!()
    }
}

#[cfg(test)] //cargo test -- --nocapture
mod tests {
    use super::Command;
    use super::Hash32;
    use super::MessageHeader;
    use super::Stream;
    use serialization::stream::Serializable;

    #[test]
    fn test_serialize() {
        let header = MessageHeader {
            magic: 1u32,
            command: Command::from_hex_string("76657273696f6e0000000000"),
            payload_size: 4u32,
            checksum: Hash32::from("6f6e0000"),
        };
        let mut stream = Stream::new();

        header.serialize(&mut stream);

        println!("{:?}",stream);
    }
}
