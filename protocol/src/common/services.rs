use serialization::stream::{Serializable, Stream};
use std::ops;
use std::ops::Deref;

pub struct Services(u64);

impl Serializable for Services {
    fn serialize(&self, s: &mut Stream) {
        s.write(&self.0);
    }

    fn serialized_size(&self) -> usize {
        8
    }
}

impl ops::Deref for Services {
    type Target = u64;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}


#[cfg(test)]
mod tests {
    use super::Services;

    #[test]
    fn test_deref() {
        let services = Services(1u64);
        assert_eq!(1u64, *services);
    }
}

