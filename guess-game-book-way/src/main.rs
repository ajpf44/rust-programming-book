use std::io;
use std::io::Write;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the guess game\nguess the random number, generated from [0 .. 100]\n");
    let aim_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        print!("guess: ");
        io::stdout().flush().expect("Error flushing");

        let mut guess_str: String = String::new();
        io::stdin()
            .read_line(&mut guess_str)
            .expect("failed to read the line");
        let guess_num: u32 = match guess_str.trim()
            .parse() {
                Ok(n) => n,
                Err(_) => continue,
            };

        println!("lets see.... your number {}", guess_num);

        match guess_num.cmp(&aim_number){
            Ordering::Less => println!("is too small"),
            Ordering::Greater => println!("is to big"),
            Ordering::Equal => {
                println!("Youu win");
                break;
            }
        }
    }
}
