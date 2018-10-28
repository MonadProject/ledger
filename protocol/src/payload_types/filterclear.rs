use super::payload::Payload;

pub struct FilterClear;

impl Payload for FilterClear{
    fn command() -> &'static str {
        "filterclear"
    }
}
