extern crate basictype;
extern crate crypto;
extern crate serialization;

pub mod block_header;
pub mod block;
pub mod transaction;
pub mod merkle_root;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
