use super::payload::Payload;

pub struct FeeFilter {
    pub feerate: u64,
}

impl Payload for FeeFilter {
    fn command() -> &'static str {
        "feefilter"
    }
}

