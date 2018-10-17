use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
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
}


#[cfg(test)]
mod tests {
    use super::Stream;
    use stream::Serializable;

    #[test]
    fn test_new_stream() {
        let new_stream = Stream::new();
        println!("{:#?}", new_stream);
        assert_eq!(new_stream, Stream {
            buffer: [].to_vec()
        });
    }

    #[test]
    fn test_bool_serialize() {
        let mut stream = Stream::new();
        true.serialize(&mut stream);
        println!("{:#?}",stream);
        assert_eq!(stream,Stream {
           buffer: [1].to_vec()
        });

        let mut stream = Stream::new();
        false.serialize(&mut stream);
        println!("{:#?}",stream);
        assert_eq!(stream,Stream {
            buffer: [0].to_vec()
        });
    }
}