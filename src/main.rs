fn main() {
    let mut vec = vec![];
    let slice = &[1u8; 10][..];
    for i in slice {
        vec.push(*i);
    }
    println!("{:?}", vec);
    println!("{:?}",slice);
}