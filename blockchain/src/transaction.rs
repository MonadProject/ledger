use basictype::bytes::Bytes;
use basictype::hash;
use serialization::reader::{Deserializable, Error, Reader};
use serialization::stream::{Serializable, Stream};
use std::io;

// see https://en.bitcoin.it/wiki/Protocol_documentation#tx

pub struct Transaction {
    pub version: i32,
    pub tx_in: Vec<Input>,
    pub tx_out: Vec<OutPoint>,
    pub witnesses: Vec<Bytes>,
}

// former transaction output, now input, more detail please google utxo pattern
pub struct OutPoint {
    pub output_hash: hash::Hash256,
    pub index: u32,
}

pub struct Input {
    pub previous_output: hash::Hash256,
    pub script_length: u64,
    //The length of the signature script
    pub signature_script: Bytes,
    pub sequence: u32,

}

pub struct Output {
    pub value: u64,
    pub pk_script_length: u64,
    pub pk_script: Bytes,
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

