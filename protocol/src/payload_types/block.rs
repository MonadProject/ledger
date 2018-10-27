//The block message is sent in response to a getdata message
// which requests transaction information from a block hash.

use blockchain::block::MonadBlock;

pub struct Block(MonadBlock);

