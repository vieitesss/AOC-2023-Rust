use std::env;

use aoc23::solve_day;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: {} <number 1-25> <input 0-2>", args[0]);
        println!("    number: the day you want to get the answer");
        println!("    input:  the input you want to use");
        println!("        - 0: full input");
        println!("        - 1: example1 input");
        println!("        - 2: example2 input");
        return;
    }

    let number: u8 = args[1].parse().unwrap_or(0);
    let input: u8 = args[2].parse().unwrap_or(0);
    
    solve_day(number, input);
}
