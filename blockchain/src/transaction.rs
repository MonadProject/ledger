use basictype::bytes::Bytes;
use basictype::hash;

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

pub struct input {
    pub previous_output: hash::Hash256,
    pub script_length: u64,
    //The length of the signature script
    pub signature_script: Bytes,
    pub sequence: u32,

}

pub struct output {
    pub value: u64,
    pub pk_script_length: u64,
    pub pk_script: Bytes,
}