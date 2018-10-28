use serialization::stream::{Serializable, Stream};

pub struct Services(u64);

impl Serializable for Services {
    fn serialize(&self, s: &mut Stream) {
        s.write(&self.0)
    }

    fn serialized_size(&self) -> usize {
        8
    }
}