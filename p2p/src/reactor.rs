use futures::{empty, Empty};
use tokio_core::reactor::Core;

pub fn reactor() -> Core {
    Core::new().unwrap()
}

pub fn forever() -> Empty<(), ()> {
    empty()
}
