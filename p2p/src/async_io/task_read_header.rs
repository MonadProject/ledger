use futures::{Async, Future, Poll};
use network::network::Magic;
use protocol::message::message_header::{MessageHeader, MessageResult};
use std::io;
use tokio::io::AsyncRead;
use tokio_io::io::{ReadExact,read_exact};


pub struct TaskReadHeader<A> {
    reader: ReadExact<A, [u8; 24]>,
    magic: Magic,
}

pub fn read_header_task<A>(a: A, magic: Magic) -> TaskReadHeader<A> where A: AsyncRead {
    TaskReadHeader {
        reader: read_exact(a, [0u8; 24]),
        magic,
    }
}

impl<A> Future for TaskReadHeader<A> where A: AsyncRead {
    type Item = (A, MessageResult<MessageHeader>);
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        unimplemented!()
    }
}

