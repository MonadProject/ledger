extern crate byteorder;
extern crate basictype;

pub mod reader;
pub mod stream;
pub mod compact;
pub mod hash;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
