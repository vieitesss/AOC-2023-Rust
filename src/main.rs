// #![feature(lazy_cell)]

use std::env;

use aoc23::solve_day;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("Usage: {} <year 2019-2023> <day 1-25> <input 0-2>", args[0]);
        println!("    year: the year you want to choose the day from");
        println!("    number: the day you want to get the answer");
        println!("    input:  the input you want to use");
        println!("        - 0: full input");
        println!("        - 1: example1 input");
        println!("        - 2: example2 input");
        return;
    }

    let year: u16 = args[1].parse().unwrap_or(0);
    let number: u8 = args[2].parse().unwrap_or(0);
    let input: u8 = args[3].parse().unwrap_or(0);
    
    match solve_day(year, number, input) {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e),
    }
}
