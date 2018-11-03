use basictype::hash::{Hash256, Hash32, Hash96};
use reader::Deserializable;
use reader::Error;
use reader::Reader;
use std::io;
use stream::Serializable;
use stream::Stream;

macro_rules! impl_hash_serialize_and_deserialize {
    ($name: ident, $size: expr) =>  {
        impl Serializable for $name {
            fn serialize(&self, s: &mut Stream) {
                s.write_slice(&**self);
            }

            fn serialized_size(&self) -> usize {
                $size
            }
        }

        impl Deserializable for $name {
            fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, Error> where Self: Sized, T: io::Read {
                let mut hash = Self::default();
                reader.read_exact(&mut *hash);
                Ok(hash)
            }

        }
    }
}

impl_hash_serialize_and_deserialize!(Hash256,32);
impl_hash_serialize_and_deserialize!(Hash96,12);
impl_hash_serialize_and_deserialize!(Hash32,4);