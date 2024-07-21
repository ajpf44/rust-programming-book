const PI:f32 = 3.14;

fn main() {
    let mut s = "this is the content s";
    println!("first value of s: {s}");

    s = "lemmon yellow sun"; 
    println!("second value of s: {s}");

    println!("printing the constant PI: {PI}");

    {
        let s = "new s shadowing in the scope";
        println!("value of s inside a scope: {s}"); 
    }

    println!("value of s after the scope: {s}");

    println!("\n============================");
    let spaces = "    ";
    let spaces = spaces.len();
    println!("numbers of spaces {}", spaces);
    println!();


    println!("\n============================");
    let tup:(i8, u32, char) = (-23, 300, '¢');
    println!("first one: {}, second: {}, and the third is {} char", tup.0, tup.1,tup.2);

    println!("\n============================");
    println!("arrays");

    let arr = [1,2,3,4,5];
    println!("the first element of the array {}", arr[0]);

    let fives_threes_arr = [3;5];
    println!("the third element for the three fives array: {}", fives_threes_arr[2]); 

    println!("\n============================");
    println!("functions");
}
