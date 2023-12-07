mod day1;
mod day2;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <number 1-25>", args[0]);
        return;
    }

    let number: u8 = args[1].parse().unwrap_or(0);

    match number {
        0 => println!("Invalid number"),
        1 => day1::resolve(),
        2 => day2::resolve(),
        _ => println!("Invalid number")
    }
}
