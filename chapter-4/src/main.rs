fn main() {

    let s = "hello"; //is a string litteral, is immutable

    let mut st = String::from(s); // is a second type of the string, is mutable
    st.push_str(", world!");

    println!("{}",st);
}
