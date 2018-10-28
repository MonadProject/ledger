//The mempool message sends a request to a node asking for information about transactions it has verified but which have not yet confirmed.
// The response to receiving this message is an inv message containing the transaction hashes for all the transactions in the node's mempool.
//
//No additional data is transmitted with this message.
//
//It is specified in BIP 35. Since BIP 37, if a bloom filter is loaded, only transactions matching the filter are replied.

pub struct MemPool;