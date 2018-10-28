//tx describes a bitcoin transaction, in reply to getdata
use blockchain::transaction::Transaction;
use super::payload::Payload;


pub struct Tx(Transaction);

impl Payload for Tx {
    fn command() -> &'static str {
        "tx"
    }
}