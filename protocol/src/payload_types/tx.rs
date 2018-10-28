//tx describes a bitcoin transaction, in reply to getdata
use blockchain::transaction::Transaction;
use super::payload::Payload;
use serialization::stream::Stream;
use serialization::reader::Error;


pub struct Tx(Transaction);

impl Payload for Tx {
    fn command() -> &'static str {
        "tx"
    }

    fn serialize(&self, stream: & mut Stream) -> Result<(), Error> {
        unimplemented!()
    }
}