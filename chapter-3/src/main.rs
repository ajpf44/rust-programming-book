//declaring const global variables
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
    log_func();
    log_ten_times("logging from another function trought parameter");


    println!("\n============================");
    println!("statements and expressions");

    let y = {
        let x = 20;
        x + 30
    };

    println!("value of y: {}", y);

    println!("\n============================");
    println!("functions with return values");
    let result: i32 = times_five(30);
    println!("times_five function: {}", result);

    let number: i32 = 2;
    println!("{number} times 5 equals to {}",  times_five(number));

    println!("\n============================");
    println!("if conditions");

    if number > 10 {
        println!("The number is highter then ten");
    }

    let new_number = if number > 10 {4} else { 10};
    println!("new_number: {}", new_number);

    
    println!("\n============================");
    println!("loops");
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let arr = ["rice", "beans", "tomato", "lettuce", "salt pile"];
    let mut index = 0;

    while index < 5 {
        print!("{index}: {} - ", arr[index]);
        index+=1;
    }
    println!();

    for s in arr {
        println!("{}",s)
    }

    println!("\n============================");
    println!("convert to fahrenheit");

    let celsius_temp: f32 = 0.0;
    
    println!("{celsius_temp}ºC in fahrenheit is {}ºF", convert_to_fahren(celsius_temp));

    let fahren_temp: f32 = 212.0;
    println!("{fahren_temp}ºF in celius is {}ºC", convert_to_celsius(fahren_temp));
                                                                
    let n = 8;
    println!("the {} fibonnaci number is {}", n, n_fibonnaci(n));

    println!("\n============================");
    println!("printing the fibonnaci sequence");
    for index in 0..10 {
        println!("{index}: {}", n_fibonnaci(index));
    }

    println!("\n============================");
    log_christmas_lyric();

}

/*
   
   [DONE]Convert temperatures between Fahrenheit and Celsius.
   [DONE]Generate the nth Fibonacci number.
    Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

*/
fn log_christmas_lyric (){
    let days_str = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh",
    "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let mut days_phrases = [
        "A partridge in a pear tree.",
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five gold rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-malking,",
        "Nine ladies dancing",
        "Ten lords a-leaping',",
        "Eleven pipers piping,",
        "Twelve drummers drumming,"
    ];
    for i in 0..12{
        println!("On the {} day of Christmas", days_str[i]);
        println!("my true love sent to me");
        for j  in 0..i {
            println!("{}", days_phrases[i-j]);
        }
        println!("And a partridge in a pear tree!");
        print!("\n=====================\n");
    }
}

fn n_fibonnaci(n: u32) -> u32{
    if n== 1 || n == 0 { return n }

    n_fibonnaci(n -1) + n_fibonnaci(n -2)
}
fn convert_to_fahren(c_temp: f32) -> f32 {
    (9.0/5.0)*c_temp + 32.0
}

fn convert_to_celsius(t_temp: f32) -> f32{
    (t_temp - 32.0) * (5.0/9.0)       
}
fn times_five(num:  i32) -> i32 {
    num*5
}
fn log_ten_times(msg: &str){
    for i in 0..10 {
        println!("{}: {msg}", i+1);
    }
}

fn log_func(){
    println!("logging from a function");
}
