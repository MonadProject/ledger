// see https://en.bitcoin.it/wiki/Protocol_documentation#block

use block_header::BlockHeader;
use transaction::Transaction;

pub struct MonadBlock {
    pub block_header: BlockHeader,
    pub transactions: Vec<Transaction>,
}
