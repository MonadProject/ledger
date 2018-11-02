#[derive(Default, Debug, PartialEq)]
pub struct Bytes {
    bytes: Vec<u8>,
}

impl Bytes {
    pub fn new() -> Self {
        Bytes::default() //关联方法
    }

    pub fn new_with_length(mut length: usize) -> Self {
        let mut vec: Vec<u8> = Vec::new();
        while length != 0 {
            vec.push(0u8);
            length = length - 1;
        }
        Bytes {
            bytes: vec
        }
    }

    pub fn take(self) -> Vec<u8> {
        self.bytes
    }

    pub fn length(&self) -> usize {
        self.bytes.len()
    }

    pub fn get_inner(&mut self) -> &mut Vec<u8> {
        &mut self.bytes
    }

    pub fn get_inner_unmut(&self) -> &Vec<u8> {
        &self.bytes
    }

    pub fn copy_from_slice(&mut self, slice: &mut [u8]) {
        for i in slice {
            self.bytes.push(*i);
        }
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(vec: Vec<u8>) -> Self {
        Bytes {
            bytes: vec
        }
    }
}


#[cfg(test)]
mod tests {
    use super::Bytes;

    #[test]
    fn test_new() {
        let b = Bytes::new();
        assert_eq!(b, Bytes {
            bytes: Vec::new()
        })
    }

    #[test]
    fn test_new_with_length() {
        let b = Bytes::new_with_length(5);
        assert_eq!(b.bytes.len(), 5);
    }
}
