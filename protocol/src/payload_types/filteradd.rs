use basictype::bytes::Bytes;
use super::payload::Payload;


pub struct FilterAdd {
    pub data: Bytes,
}

impl Payload for FilterAdd {
    fn command() -> &'static str {
        "filteradd"
    }
}