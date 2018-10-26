use socket::peer::Peer;
use socket::peer::PeerId;
use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::Arc;

//代表与对等节点的一个连接
struct Channel {
    stream: TcpStream,
    peer: Peer,
}

struct Connections {
    channels: HashMap<PeerId, Arc<Channel>>,
}