extern crate basictype;
extern crate crypto as mcrypto;

use basictype::hash::Hash256;
use mcrypto::digest::Digest;
use mcrypto::sha2::Sha256;


//do not use these two methods
pub fn sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(input);
    hasher.result_str()
}

pub fn double_sha256(input: &str) -> String {
    sha256(&sha256(input)[..])
}


pub fn sha2(input: &[u8]) -> Hash256 {
    let mut hasher = Sha256::new();
    let mut result = Hash256::default();
    hasher.input(input);
    hasher.result(&mut *result);
    result
}

pub fn double_sha2(input: &[u8]) -> Hash256 {
    let h2 = sha2(input);
    let mut hasher = Sha256::new();
    let mut result = Hash256::default();
    hasher.input(&*h2);
    hasher.result(&mut *result);
    result
}





#[cfg(test)]
mod tests {
    use double_sha256;
    use sha2;
    use sha256;
    use double_sha2;

    #[test]
    fn test_sha2() {
        let slice = b"hello";
        let h256 = sha2(&slice[..]);
        println!("{:?}", h256);
    }

    #[test]
    fn test_double_sha2() {
        let slice = b"hello";
        let dh256 = double_sha2(&slice[..]);
        println!("{:?}", dh256);
        assert_eq!(dh256.to_string(),"9595c9df90075148eb06860365df33584b75bff782a510c6cd4883a419833d50");
    }


    #[test]
    fn test_sha256() {
        let out = sha256("hello");
        let expected = String::from("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
        println!("{}", out);
        assert_eq!(out, expected)
    }

    #[test]
    fn test_double_sha256() {
        let out = double_sha256("hello");
        let expected = String::from("d7914fe546b684688bb95f4f888a92dfc680603a75f23eb823658031fff766d9");
        println!("{}", out);
        assert_eq!(out, expected)
    }
}
