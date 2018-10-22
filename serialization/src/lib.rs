extern crate byteorder;
extern crate basictype;

pub mod reader;
pub mod stream;
pub mod compact;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
