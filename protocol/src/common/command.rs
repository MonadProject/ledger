use basictype::hash::Hash96;
use serialization::reader::Deserializable;
use serialization::stream::{Serializable, Stream};

pub struct Command(Hash96);

impl Serializable for Command {
    fn serialize(&self, s: &mut Stream) {
        s.write(&self.0);
    }

    fn serialized_size(&self) -> usize {
        12
    }
}

#[cfg(test)] //cargo test -- --nocapture
mod tests {
    use serialization::stream::Serializable;
    use std::str::FromStr;
    use super::Command;
    use super::Hash96;
    use super::Stream;

    #[test]
    fn test_serialize() {
        let string = "76657273696f6e0000000000";
        let hash = Hash96::from_str(string).unwrap();
        let command = Command(hash);
        let mut stream = Stream::new();
        command.serialize(&mut stream);
        println!("{:?}", stream);
    }
}