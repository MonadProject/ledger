//tx describes a bitcoin transaction, in reply to getdata
use blockchain::transaction::Transaction;

pub struct Tx(Transaction);