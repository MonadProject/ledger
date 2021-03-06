use basictype::bytes::Bytes;
use basictype::hash;
use serialization::compact::Compact;
use serialization::reader::{Deserializable, Error, Reader};
use serialization::stream::{Serializable, Stream};
use std::io;

// see https://en.bitcoin.it/wiki/Protocol_documentation#tx
#[derive(Debug)]
pub struct Transaction {
    pub version: i32,
    pub tx_in: Vec<Input>,
    pub tx_out: Vec<OutPoint>,
    pub witnesses: Vec<Bytes>,
    pub lock_time: u32,
}

impl Serializable for Transaction {
    fn serialize(&self, s: &mut Stream) {
        s.write(&self.version);
        s.write_list(&self.tx_in);
        s.write_list(&self.tx_out);
        s.write_list(&self.witnesses);
        s.write(&self.lock_time);
    }

    fn serialized_size(&self) -> usize {
        unimplemented!()
    }
}

impl Deserializable for Transaction {
    fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, Error> where Self: Sized, T: io::Read {
        Ok(Transaction {
            version: reader.read()?,
            tx_in: reader.read()?,
            tx_out: reader.read()?,
            witnesses: reader.read()?,
            lock_time: reader.read()?,
        })
    }
}

// former transaction output, now input, more detail please google utxo pattern
#[derive(Debug)]
pub struct OutPoint {
    pub output_hash: hash::Hash256,
    pub index: u32,
}

impl OutPoint {
    fn new() -> Self {
        OutPoint {
            output_hash: hash::Hash256::default(),
            index: 0u32,
        }
    }
}


impl Serializable for OutPoint {
    fn serialize(&self, s: &mut Stream) {
        s.write(&self.output_hash);
        s.write(&self.index);
    }

    fn serialized_size(&self) -> usize {
        36
    }
}

impl Deserializable for OutPoint {
    fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, Error> where Self: Sized, T: io::Read {
        let outpoint = OutPoint {
            output_hash: reader.read::<hash::Hash256>()?,
            index: reader.read::<u32>()?,
        };
        Ok(outpoint)
    }
}

#[derive(Debug)]
pub struct Input {
    pub previous_output: OutPoint,
    pub signature_script: Bytes,
    pub sequence: u32,
}


impl Serializable for Input {
    fn serialize(&self, s: &mut Stream) {
        s.write(&self.previous_output);
        s.write(&self.signature_script);
        s.write(&self.sequence);
    }

    fn serialized_size(&self) -> usize {
        unimplemented!()
    }
}

impl Deserializable for Input {
    fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, Error> where Self: Sized, T: io::Read {
        let input = Input {
            previous_output: reader.read()?,
            signature_script: reader.read()?,
            sequence: reader.read()?,
        };

        Ok(input)
    }
}


pub struct Output {
    pub value: u64,
    pub pk_script: Bytes,
}

impl Serializable for Output {
    fn serialize(&self, s: &mut Stream) {
        s.write(&self.value);
        s.write(&self.pk_script);
    }

    fn serialized_size(&self) -> usize {
        8 + Compact::from(self.pk_script.length()).serialized_size()
    }
}

impl Deserializable for Output {
    fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, Error> where Self: Sized, T: io::Read {
        let output = Output {
            value: reader.read()?,
            pk_script: reader.read()?,
        };
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use serialization::compact;
    use serialization::reader::Deserializable;
    use serialization::stream::Serializable;
    use super::{Error, hash, OutPoint, Reader, Stream};

    #[test]
    fn test_outpoint_serializable() {
        let hex_string: &'static str = "0000000000000000000383fb0c96397da185a378d04cf7d451ef81a7b446fbb7";

        let outpoint = OutPoint {
            output_hash: hash::Hash256::from_reversed_string(hex_string),
            index: 1u32,
        };

        let mut stream = Stream::new();

        outpoint.serialize(&mut stream);

        println!("{:?}", stream);
    }

    #[test]
    fn test_outpoint_deserialize() {
        let buffer: Vec<u8> = vec![183, 251, 70, 180, 167, 129, 239, 81, 212, 247, 76, 208, 120, 163, 133, 161, 125, 57, 150, 12, 251, 131, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0];
        let mut reader = Reader::from_bytes(&buffer);
        let outpoint = OutPoint {
            output_hash: reader.read().unwrap(),
            index: reader.read().unwrap(),
        };

        println!("{:?}", outpoint);

        assert_eq!(outpoint.output_hash.to_reversed_string(), "0000000000000000000383fb0c96397da185a378d04cf7d451ef81a7b446fbb7");
    }
}

