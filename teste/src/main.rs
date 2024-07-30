fn main() {
    let arr_str: [&str; 4] = ["Hello World";4];

    for s in arr_str {
        println!("{}",s);
    }
}
