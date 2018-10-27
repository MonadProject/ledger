use futures::{Async, Future, Poll};
use network::network::Magic;
use protocol::message::message_header::{MessageHeader, MessageResult};
use serialization::reader::Deserializable;
use std::io;
use tokio_io::AsyncRead;
use tokio_io::io::{read_exact, ReadExact};


pub struct FutureReadHeader<A> {
    reader: ReadExact<A, [u8; 24]>,
    magic: Magic,
}

pub fn read_header_task<A>(a: A, magic: Magic) -> FutureReadHeader<A> where A: AsyncRead {
    FutureReadHeader {
        reader: read_exact(a, [0u8; 24]),
        magic,
    }
}

impl<A> Future for FutureReadHeader<A> where A: AsyncRead {
    type Item = (A, MessageResult<MessageHeader>);
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.reader.poll() {
            Ok(Async::Ready((reader, data))) => {
                let header = MessageHeader::deserialize_slice(&data);
                Ok(Async::Ready((reader, header)))
            }

            Ok(Async::NotReady) => {
                Ok(Async::NotReady)
            }

            Err(e) => {
                Err(e)
            }
        }
    }
}

