struct Hello(u8);

fn main() {
    let hello = Hello(1u8);
    match  hello.0 {
        i => { println!("{}",i)}
    }
}