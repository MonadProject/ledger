extern crate byteorder;
extern crate basictype;

mod reader;
mod stream;
mod compact;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
