use stream::Serializable;
use stream::Stream;

pub struct Compact(u64);

impl From<u8> for Compact {
    fn from(i: u8) -> Self {
        Compact(i as u64)
    }
}

impl From<u16> for Compact {
    fn from(i: u16) -> Self {
        Compact(i as u64)
    }
}

impl From<u32> for Compact {
    fn from(i: u32) -> Self {
        Compact(i as u64)
    }
}

impl From<u64> for Compact {
    fn from(i: u64) -> Self {
        Compact(i)
    }
}

impl Serializable for Compact {
    fn serialize(&self, s: &mut Stream) {
        unimplemented!()
    }

    fn serialized_size(&self) -> usize {
        unimplemented!()
    }
}