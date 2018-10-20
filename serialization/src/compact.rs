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
        match self.0 {
            0...0xfd => {
                s.write_struct(&(self.0 as u8));
            }
            0xfd...0xffff => {
                s.write_struct(&0xfd);
                s.write_struct(&(self.0 as u16));
            }
            0xffff...0xffff_ffff => {
                s.write_struct(&0xfe);
                s.write_struct(&(self.0 as u32));
            }
            _ => {
                s.write_struct(&0xff);
                s.write_struct(&(self.0 as u64));
            }
        }
    }

    fn serialized_size(&self) -> usize {
        match self.0 {
            0...0xfd => 1,
            0xfd...0xffff => 3,
            0xffff...0xffff_ffff => 5,
            _ => 9
        }
    }
}