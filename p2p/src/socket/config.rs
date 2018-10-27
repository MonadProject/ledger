use std::net::SocketAddr;
use std::net;
use std::path;

pub struct Services(u64);
pub type Magic = u32;

pub struct NetWorkConfig {
    pub protocol_version: u32,
    pub protocol_minimum: u32,
    pub magic: Magic,
    pub local_address: SocketAddr,
    pub services: Services,
    pub user_agent: String,
    pub start_height: i32,
    pub relay: bool,
}

pub enum InternetProtocol {
    Any,
    IpV4,
    IpV6,
}

pub struct Config {
    pub threads: usize,
    pub inbound_connections: usize,
    pub outbound_connections: usize,
    pub peers: Vec<net::SocketAddr>,
    /// Connect to these nodes to retrieve peer addresses, and disconnect.
    pub seeds: Vec<String>,
    /// p2p/nodes.csv file path.
    pub node_table_path: path::PathBuf,
    /// Peers with this Services will get a boost in node_table.
    pub preferable_services: Services,
    /// Internet protocol.
    pub internet_protocol: InternetProtocol,
}