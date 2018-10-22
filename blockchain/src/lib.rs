extern crate basictype;
extern crate crypto;
extern crate serialization;

mod block_header;
mod block;
mod transaction;
mod merkle_root;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
