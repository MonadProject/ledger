//see https://en.bitcoin.it/wiki/Protocol_documentation#getheaders

//Return a headers packet containing the headers of blocks starting right after
// the last known hash in the block locator object, up to hash_stop or 2000 blocks,
// whichever comes first. To receive the next block headers,
// one needs to issue getheaders again with a new block locator object.
// Keep in mind that some clients may provide headers of blocks which are invalid if the block locator object
// contains a hash on the invalid branch.

use payload_types::version::Version;
use basictype::hash::Hash256;

pub struct GetHeaders{
    pub version: Version,
    pub block_locator_hashes: Hash256,
    pub hash_stop: Hash256,
}