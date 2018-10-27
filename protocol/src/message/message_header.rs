use basictype::hash::Hash32;
use common::command::Command;
use network::network::Magic;
use serialization::reader::Deserializable;
use serialization::reader::Error;
use serialization::reader::Reader;
use serialization::stream::{Serializable, Stream};
use std::io;


//see https://bitcoin.org/en/developer-reference#message-headers
#[derive(Debug, PartialEq)]
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

impl Default for MessageHeader {
    fn default() -> Self {
        MessageHeader {
            magic: 0u32,
            command: Command::default(),
            payload_size: 032,
            checksum: Hash32::default(),
        }
    }
}


impl Deserializable for MessageHeader {
    fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, Error> where Self: Sized, T: io::Read {
        let header = MessageHeader {
            magic: reader.read()?,
            command: reader.read()?,
            payload_size: reader.read()?,
            checksum: reader.read()?,
        };

        Ok(header)
    }
}

impl MessageHeader {
    pub fn deserialize_slice(buffer: &[u8]) -> Result<Self, Error> {
        let mut reader = Reader::from_bytes(buffer);
        Deserializable::deserialize(&mut reader)
    }
}

#[cfg(test)] //cargo test -- --nocapture
mod tests {
    use serialization::reader::Deserializable;
    use serialization::stream::Serializable;
    use super::Command;
    use super::Hash32;
    use super::MessageHeader;
    use super::Reader;
    use super::Stream;

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

        println!("{:?}", stream);
    }

    #[test]
    fn test_deserialize() {
        let buffer = [1u8, 0, 0, 0, 118, 101, 114, 115, 105, 111, 110, 0, 0, 0, 0, 0, 4, 0, 0, 0, 111, 110, 0, 0];
        let mut reader = Reader::from_bytes(&buffer);
        let header = MessageHeader::deserialize(&mut reader).unwrap();
        println!("{:?}", header);
        let expected_header = MessageHeader {
            magic: 1u32,
            command: Command::from_hex_string("76657273696f6e0000000000"),
            payload_size: 4u32,
            checksum: Hash32::from("6f6e0000"),
        };
        assert_eq!(expected_header, header);
    }
}
