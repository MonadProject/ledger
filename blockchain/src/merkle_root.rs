use basictype::hash::{Hash256, Hash512};
use crypto::double_sha256;

fn concatenate<T>(a: T, b: T) -> Hash512 where T: AsRef<Hash256> {
    let mut h521 = Hash512::default();
    h521[0..32].copy_from_slice(&**a.as_ref());
    h521[32..64].copy_from_slice(&**b.as_ref());
    h521
}

fn calculate_merge_hash<T>(a: T, b:T) -> Hash256 where T: AsRef<Hash256> {

}