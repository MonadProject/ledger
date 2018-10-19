fn main() {
    let r = f1(String::from("aa"), 1u8);
    println!("{}", r)
}


fn f1<A, B>(a: A, b: B) -> B {
    f2(b)
}

fn f2<A>(b: A) -> A {
    b
}