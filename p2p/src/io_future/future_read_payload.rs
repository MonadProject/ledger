use basictype::bytes::Bytes;
use basictype::hash::Hash32;
use protocol::payload::Payload;
use serialization::reader::Deserializable;
use std::io;
use tokio_io::AsyncRead;
use tokio_io::io;
use tokio_io::io::{read_exact, ReadExact};


pub struct FutureReadPayload<A, P> where A: AsyncRead, P: Payload {
    reader: ReadExact<A, Bytes>,
    checksum: Hash32,
    payload: P
}