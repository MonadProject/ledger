//These messages are related to Bloom filtering of connections and are defined in BIP 0037.


use basictype::bytes::Bytes;
use super::payload::Payload;


pub struct FilterLoad {
    //The filter itself is simply a bit field of arbitrary byte-aligned size. The maximum size is 36,000 bytes.
    pub filter: Bytes,
    //The number of hash functions to use in this filter. The maximum value allowed in this field is 50.
    pub nHashFuncs: u32,
    //A random value to add to the seed value in the hash function used by the bloom filter.
    pub nTweak: u32,
    //A set of flags that control how matched items are added to the filter.
    pub nFlags: u8,

}

impl Payload for FilterLoad {
    fn command() -> &'static str {
        "filterload"
    }
}