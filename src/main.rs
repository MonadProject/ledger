fn main() {
    let a = vec![String::from("aa")];
    let b = &&a[0];
    let b = &&a[0];
    println!("{}",b);

}