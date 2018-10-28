pub trait Payload {
    fn command() -> &'static str;
}