use std::net::SocketAddr;

enum Direction {
    Inbound,
    OutBound,
}

pub type PeerId = usize;

pub struct Peer {
    id: PeerId,
    address: SocketAddr,
}