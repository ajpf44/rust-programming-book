use std::time::{Instant};
use std::env;
fn main() {
    let start = Instant::now();
    let mut duration:u64  = start.elapsed().as_secs();

    let mut args = env::args().skip(1);
    let arg_num: u64 = args
        .next()
        .unwrap()
        .trim().parse().expect("Error parsing");

    while duration < arg_num {
        let new_duration:u64  =start.elapsed().as_secs();
        
        if new_duration != duration {
            println!("{}/{}", new_duration, arg_num);
            duration = new_duration;
        }

    }

    println!("finished");
}

