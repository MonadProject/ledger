use byteorder::ReadBytesExt;
use std::io;
use std::marker;


pub struct Reader<T> {
    inner: T
}

pub enum Error {}

pub trait Deserializable {
    fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, Error> where Self: Sized, T: io::Read;
}

impl<T> io::Read for Reader<T> where T: io::Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        io::Read::read(&mut self.inner, buf)
    }
}


impl Deserializable for u8 {
    fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, Error> where Self: Sized, T: io::Read {
        Ok(reader.read_u8().unwrap())
    }
}

