use basictype::hash::Hash96;
use serialization::reader::Deserializable;
use serialization::reader::Error;
use serialization::reader::Reader;
use serialization::stream::{Serializable, Stream};
use std::io;
use std::str::FromStr;


#[derive(Debug,PartialEq)]
pub struct Command(Hash96);

impl Serializable for Command {
    fn serialize(&self, s: &mut Stream) {
        s.write(&self.0);
    }

    fn serialized_size(&self) -> usize {
        12
    }
}

impl Deserializable for Command {
    fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, Error> where Self: Sized, T: io::Read {
        let hash = reader.read::<Hash96>().unwrap();
        Ok(Command(hash))
    }
}

impl Default for Command {
    fn default() -> Self {
        Command(Hash96::default())
    }
}

impl Command {
    pub fn from_hex_string(string: &str) -> Self {
        let hash = Hash96::from_str(&string).unwrap();
        Command(hash)
    }
}

#[cfg(test)] //cargo test -- --nocapture
mod tests {
    use serialization::reader::Deserializable;
    use serialization::stream::Serializable;
    use std::str::FromStr;
    use super::Command;
    use super::Hash96;
    use super::Reader;
    use super::Stream;

    #[test]
    fn test_from_hex_string() {
        let string = String::from("76657273696f6e0000000000");
        let command = Command::from_hex_string(&string);
        println!("{:?}", command);
    }

    #[test]
    fn test_serialize() {
        let string = "76657273696f6e0000000000";
        let hash = Hash96::from_str(string).unwrap();
        let command = Command(hash);
        let mut stream = Stream::new();
        command.serialize(&mut stream);
        println!("{:?}", stream);
    }

    #[test]
    fn test_deserialize() {
        let buffer = [118u8, 101, 114, 115, 105, 111, 110, 0, 0, 0, 0, 0];
        let mut reader = Reader::from_bytes(&buffer);
        let result = Command::deserialize(&mut reader).unwrap();
        println!("{:?}", result);
    }
}