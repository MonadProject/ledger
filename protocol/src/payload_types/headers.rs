//The headers packet returns block headers in response to a getheaders packet.

use blockchain::block_header::BlockHeader;

pub struct Headers(Vec<BlockHeader>);