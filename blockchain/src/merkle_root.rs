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
    let mut index = 0;
    while index + 1 < input.len() {
        let node = calculate_merge_hash(input[index].as_ref(), input[index + 1].as_ref());
        vec.push(node);
        index += 2;
    }

    if input.len() % 2 != 0 {
        let node = calculate_merge_hash(&input[input.len() - 1], &input[input.len() - 1]);
        vec.push(node);
    }

    calculate_merge_root(&vec)
}

#[cfg(test)]
mod tests {
    use super::calculate_merge_root;
    use super::Hash256;
    use super::Hash512;

    #[test]
    fn test_one_element() {
        let hash = Hash256::from_reversed_string("c06fbab289f723c6261d3030ddb6be121f7d2508d77862bb1e484f5cd7f92b25");
        let vec = vec![hash];
        let result = calculate_merge_root(&vec);
        println!("{}", result.to_reversed_string());
        assert_eq!("c06fbab289f723c6261d3030ddb6be121f7d2508d77862bb1e484f5cd7f92b25", result.to_reversed_string());
    }

    #[test]
    fn test_two_elements() {
        let hash_a = Hash256::from_reversed_string("c06fbab289f723c6261d3030ddb6be121f7d2508d77862bb1e484f5cd7f92b25");
        let hash_b = Hash256::from_reversed_string("5a4ebf66822b0b2d56bd9dc64ece0bc38ee7844a23ff1d7320a88c5fdb2ad3e2");
        let vec = vec![hash_a, hash_b];
        let result = calculate_merge_root(&vec);
        println!("{}", result.to_reversed_string());
        assert_eq!("8fb300e3fdb6f30a4c67233b997f99fdd518b968b9a3fd65857bfe78b2600719",result.to_reversed_string());
    }
}