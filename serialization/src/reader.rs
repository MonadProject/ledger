use byteorder::{LittleEndian, ReadBytesExt};
use std::io;
use std::marker;


pub struct Reader<T> {
    inner: T
}

pub enum Error {
    ReadDataErr
}

pub trait Deserializable {
    fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, Error> where Self: Sized, T: io::Read;
}


impl<R> Reader<R> where R: io::Read {
    pub fn from_buffer(buffer: R) -> Self {
        Reader {
            inner: buffer
        }
    }

    pub fn read<T>(&mut self) -> Result<T, Error> where T: Deserializable {
        T::deserialize(self)
    }
}


impl Deserializable for u8 {
    fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, Error> where Self: Sized, T: io::Read {
        Ok(reader.read_u8().unwrap())
    }
}


impl Deserializable for u16 {
    fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, Error> where Self: Sized, T: io::Read {
        Ok(reader.read_u16::<LittleEndian>().unwrap())
    }
}

impl Deserializable for u32 {
    fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, Error> where Self: Sized, T: io::Read {
        Ok(reader.read_u32::<LittleEndian>().unwrap())
    }
}

impl Deserializable for i8 {
    fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, Error> where Self: Sized, T: io::Read {
        Ok(reader.read_i8().unwrap())
    }
}

impl Deserializable for i16 {
    fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, Error> where Self: Sized, T: io::Read {
        Ok(reader.read_i16::<LittleEndian>().unwrap())
    }
}

impl Deserializable for i32 {
    fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, Error> where Self: Sized, T: io::Read {
        Ok(reader.read_i32::<LittleEndian>().unwrap())
    }
}

impl Deserializable for f32 {
    fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, Error> where Self: Sized, T: io::Read {
        Ok(reader.read_f32::<LittleEndian>().unwrap())
    }
}

impl Deserializable for f64 {
    fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, Error> where Self: Sized, T: io::Read {
        Ok(reader.read_f64::<LittleEndian>().unwrap())
    }
}

pub fn deserialize<R, T>(buffer: R) -> Result<T, Error> where R: io::Read, T: Deserializable {
    let mut reader = Reader::from_buffer(buffer);
    reader.read::<T>()
//    reader.read()
}


// 为了得到 ReadBytesExt 的能力
impl<T> io::Read for Reader<T> where T: io::Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        io::Read::read(&mut self.inner, buf)
    }
}

impl<'a> Reader<&'a [u8]> {
    fn from_bytes(bytes: &'a [u8]) -> Self {
        Reader {
            inner: bytes
        }
    }
}

#[cfg(test)] //cargo test -- --nocapture
mod tests {
    use super::deserialize;

    #[test]
    fn test_deserialize() {
        let r = deserialize::<&[u8], u8>(&[1u8]).ok().unwrap();
        println!("{}", r);
        assert_eq!(1, r);

        let r = deserialize::<&[u8], u16>(&[144, 1]).ok().unwrap();
        println!("{}", r);
        assert_eq!(400, r);
    }
}