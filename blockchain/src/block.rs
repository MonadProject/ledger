// see https://en.bitcoin.it/wiki/Protocol_documentation#block

use block_header::BlockHeader;
use transaction::Transaction;

#[derive(Monad_Serializable, Monad_Deserializable, Debug)]
pub struct MonadBlock {
    pub block_header: BlockHeader,
    pub transactions: Vec<Transaction>,
}
