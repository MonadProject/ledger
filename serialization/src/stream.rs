use byteorder::{LittleEndian, WriteBytesExt};
use std::io::Error;
use std::io::Write;


pub trait Serializable {
    fn serialize(&self, s: &mut Stream);

    //byte length
    fn serialized_size(&self) -> usize;
}

impl Serializable for bool {
    fn serialize(&self, s: &mut Stream) {
        s.write_u8(*self as u8).unwrap()
    }


    fn serialized_size(&self) -> usize {
        1
    }
}

impl Serializable for u8 {
    fn serialize(&self, s: &mut Stream) {
        s.write_u8(*self).unwrap()
    }

    fn serialized_size(&self) -> usize {
        1
    }
}

impl Serializable for u16 {
    fn serialize(&self, s: &mut Stream) {
        s.write_u16::<LittleEndian>(*self).unwrap()
    }

    fn serialized_size(&self) -> usize {
        2
    }
}

impl Serializable for u32 {
    fn serialize(&self, s: &mut Stream) {
        s.write_u32::<LittleEndian>(*self).unwrap()
    }

    fn serialized_size(&self) -> usize {
        4
    }
}

impl Serializable for i8 {
    fn serialize(&self, s: &mut Stream) {
        s.write_i8(*self).unwrap()
    }

    fn serialized_size(&self) -> usize {
        1
    }
}

impl Serializable for i16 {
    fn serialize(&self, s: &mut Stream) {
        s.write_i16::<LittleEndian>(*self).unwrap()
    }

    fn serialized_size(&self) -> usize {
        2
    }
}

impl Serializable for i32 {
    fn serialize(&self, s: &mut Stream) {
        s.write_i32::<LittleEndian>(*self).unwrap()
    }

    fn serialized_size(&self) -> usize {
        8
    }
}

#[derive(Debug, PartialEq)]
pub struct Stream {
    buffer: Vec<u8>
}

/// All types that implement `Write` get methods defined in `WriteBytesExt`
/// for free.
impl Write for Stream {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        self.buffer.write(buf)
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.buffer.flush()
    }
}

impl Stream {
    pub fn new() -> Self {
        Stream {
            buffer: Vec::new()
        }
    }

    pub fn write_slice(&mut self, bytes: &[u8]) -> &mut Self {
        self.buffer.write(bytes).unwrap();
        self
    }
}


#[cfg(test)]
mod tests {
    use stream::Serializable;
    use super::Stream;

    #[test]
    fn test_new_stream() {
        let new_stream = Stream::new();
        println!("{:#?}", new_stream);
        assert_eq!(new_stream, Stream {
            buffer: [].to_vec()
        });
    }

    #[test]
    fn test_u8_serialize() {
        let mut stream = Stream::new();
        1u8.serialize(&mut stream);
        0u8.serialize(&mut stream);
        println!("{:#?}", stream);
    }

    #[test]
    fn test_u16_serialize() {
        let mut stream = Stream::new();
        400u16.serialize(&mut stream);
        println!("{:#?}", stream);
    }

    #[test]
    fn test_bool_serialize() {
        let mut stream = Stream::new();
        true.serialize(&mut stream);
        println!("{:#?}", stream);
        assert_eq!(stream, Stream {
            buffer: [1].to_vec()
        });

        let mut stream = Stream::new();
        false.serialize(&mut stream);
        println!("{:#?}", stream);
        assert_eq!(stream, Stream {
            buffer: [0].to_vec()
        });
    }
}