use common::network_address::Network_Address;
use payload::Payload;
use serialization::stream::Stream;
use serialization::reader::Error;

pub enum Version {
    V0(Version0),
    V106(Version0, Version106),
    V70001(Version0, Version106, Version70001),
}

pub struct Version0 {
    pub version: u32,
    //bitfield of features to be enabled for this connection
    pub services: u64,
    pub timestamp: u64,
    pub addr_recv: Network_Address,//The network address of the node receiving this message
}

pub struct Version106 {
    //The network address of the node emitting this message
    pub addr_from: Network_Address,
    pub nonce: u64,
    pub user_agent: String,
    pub start_height: u32,
}

pub struct Version70001 {
    //Whether the remote peer should announce relayed transactions or not
    pub relay: bool,
}

impl Payload for Version {
    fn command() -> &'static str {
        "version"
    }

    fn serialize(&self, stream: &mut Stream) -> Result<(), Error> {
        unimplemented!()
    }
}