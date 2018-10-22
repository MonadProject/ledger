use basictype::bytes::Bytes;
use byteorder::{LittleEndian, ReadBytesExt};
use compact::Compact;
use std::io;
use std::io::Write;
use std::marker;


pub struct Reader<T> {
    inner: T
}

#[derive(Debug, PartialEq)]
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

    pub fn read_exact(&mut self, buf: &mut [u8]) {
        io::Read::read_exact(self, buf);
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

impl Deserializable for u64 {
    fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, Error> where Self: Sized, T: io::Read {
        Ok(reader.read_u64::<LittleEndian>().unwrap())
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

//todo: feel something wrong, please test it carefully!!
impl Deserializable for Bytes {
    fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, Error> where Self: Sized, T: io::Read {
        let compact = reader.read::<Compact>()?;
        let mut bytes = Bytes::new_with_length(compact.into());
        reader.read_exact(bytes.get_inner().as_mut_slice());

//        let mut buf = &mut [0u8][..];
//        reader.read_exact(buf);
//        bytes.copy_from_slice(buf);
        Ok(bytes)
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
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        Reader {
            inner: bytes
        }
    }
}

#[cfg(test)] //cargo test -- --nocapture
mod tests {
    use super::Bytes;
    use super::Deserializable;
    use super::deserialize;
    use super::Reader;

    #[test]
    fn test_deserialize() {
        let r = deserialize::<&[u8], u8>(&[1u8]).ok().unwrap();
        println!("{}", r);
        assert_eq!(1, r);

        let r = deserialize::<&[u8], u16>(&[144, 1]).ok().unwrap();
        println!("{}", r);
        assert_eq!(400, r);
    }

    #[test]
    fn test_bytes_test_deserialize() {
        let buf = [6u8, 1, 0, 1, 1, 1, 0];
        let mut reader = Reader::from_bytes(&buf);
        let result = Bytes::deserialize(&mut reader).unwrap();
        println!("{:?}", result);
        assert_eq!(vec![1, 0, 1, 1, 1, 0], result.take());
    }
}