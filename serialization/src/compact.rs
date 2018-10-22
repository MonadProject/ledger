use reader::Deserializable;
use reader::Error;
use reader::Reader;
use std::io;
use stream::Serializable;
use stream::Stream;
use basictype::hash::Hash256;

pub struct Compact(u64);

impl From<u8> for Compact {
    fn from(i: u8) -> Self {
        Compact(i as u64)
    }
}

impl From<u16> for Compact {
    fn from(i: u16) -> Self {
        Compact(i as u64)
    }
}

impl From<u32> for Compact {
    fn from(i: u32) -> Self {
        Compact(i as u64)
    }
}

impl From<u64> for Compact {
    fn from(i: u64) -> Self {
        Compact(i)
    }
}

impl From<usize> for Compact {
    fn from(i: usize) -> Self {
        Compact(i as u64)
    }
}

impl Serializable for Compact {
    fn serialize(&self, s: &mut Stream) {
        match self.0 {
            0...0xfc => {
                s.write_struct(&(self.0 as u8));
            }
            0xfd...0xffff => {
                s.write_struct(&0xfd);
                s.write_struct(&(self.0 as u16));
            }
            0xffff...0xffff_ffff => {
                s.write_struct(&0xfe);
                s.write_struct(&(self.0 as u32));
            }
            _ => {
                s.write_struct(&0xff);
                s.write_struct(&(self.0 as u64));
            }
        }
    }

    fn serialized_size(&self) -> usize {
        match self.0 {
            0...0xfc => 1,
            0xfd...0xffff => 3,
            0xffff...0xffff_ffff => 5,
            _ => 9
        }
    }
}

impl Deserializable for Compact {
    fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, Error> where Self: Sized, T: io::Read {
        let compact = match reader.read::<u8>()? {
            i @ 0...0xfc => i.into(),
            0xfd => reader.read::<u16>()?.into(),
            0xfe => reader.read::<u32>()?.into(),
            _ => reader.read::<u64>()?.into(),
        };

        Ok(compact)
    }
}

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