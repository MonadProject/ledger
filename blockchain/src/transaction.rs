use basictype::bytes::Bytes;
use basictype::hash;
use serialization::stream::Serializable;
use serialization::stream::Stream;

// see https://en.bitcoin.it/wiki/Protocol_documentation#tx

pub struct Transaction {
    pub version: i32,
    pub tx_in: Vec<input>,
    pub tx_out: Vec<output>,
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
        s.write_struct(self.output_hash);
        s.write_struct(self.index);
    }

    fn serialized_size(&self) -> usize {
        36
    }
}

