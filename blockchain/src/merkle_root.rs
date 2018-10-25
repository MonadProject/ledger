use basictype::hash::{Hash256, Hash512};
use crypto::double_sha2;

fn concatenate<T>(a: T, b: T) -> Hash512 where T: AsRef<Hash256> {
    let mut h521 = Hash512::default();
    h521[0..32].copy_from_slice(&**a.as_ref());
    h521[32..64].copy_from_slice(&**b.as_ref());
    h521
}

fn calculate_merge_hash<T>(a: T, b: T) -> Hash256 where T: AsRef<Hash256> {
    double_sha2(&*concatenate(a, b))
}

// see https://en.bitcoin.it/wiki/Protocol_documentation#Merkle_Trees
fn calculate_merge_root<T>(input: &[T]) -> Hash256 where T: AsRef<Hash256> {
    if input.len() == 1 {
        return input[0].as_ref().clone();
    }
    let mut vec = Vec::new();
    let index = 0;
    while index + 1 < input.len() {
        let node = calculate_merge_hash(input[index].as_ref(), input[index + 1].as_ref());
        vec.push(node);
    }

    if input.len() % 2 != 0 {
        let node = calculate_merge_hash(&input[input.len() - 1], &input[input.len() - 1]);
        vec.push(node);
    }

    calculate_merge_root(&vec)
}