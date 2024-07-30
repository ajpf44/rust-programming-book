use std::io;
use rand::Rng;
use std::io::Write;

fn main() {
    println!("starting the guess game\nguess a number in a range 0-100");
    let mut rng  = rand::thread_rng();
    let guess_num:u32 = rng.gen_range(0..100);
    let mut tries: u8 = 1;

    while tries < 8 {
        print!("type your guess: ");
        io::stdout().flush().expect("Invalid flush"); // to refresh the outer

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Falied to read the line");

        let player_num:u32 = input
            .trim()
            .parse()
            .expect("Not a number");


        if guess_num > player_num {
            println!("the number is greater");
        }else if guess_num < player_num {
            println!("the number is shorter");
        }else { break;}

        tries += 1;
    }

    if tries < 8 {
        println!("You found the number in {} tries", tries);
    } else {
        println!("you got without tries. Loooser");
    }
}
