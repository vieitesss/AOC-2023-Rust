#![allow(unused_comparisons)]
use std::fs;

const INPUT_PATH: &str = "data/day1/input.txt";
const _EXAMPLE1_PATH: &str = "data/day1/example1.txt";
const _EXAMPLE2_PATH: &str = "data/day1/example2.txt";

pub fn resolve() {
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
}

fn part_1() -> u32 {
    let mut sum: u32 = 0;

    for line in fs::read_to_string(INPUT_PATH).unwrap().lines() {
        let number = parse_line_1(line);
        sum += number;
    }

    sum
}

fn parse_line_1(line: &str) -> u32 {
    let mut result: u32 = 0;
    let mut last_number: u32 = 0;
    let mut last: bool = false;

    for c in line.chars() {
        if c.is_numeric() {
            if !last {
                result += 10 * c.to_digit(10).unwrap();
                last = true;
            }
            last_number = c.to_digit(10).unwrap();
        }
    }

    result += last_number;

    result
}

fn part_2() -> u32 {
    let mut sum: u32 = 0;

    for line in fs::read_to_string(INPUT_PATH).unwrap().lines() {
        let number = parse_line_2(line);
        sum += number;
    }

    sum
}

fn parse_line_2(line: &str) -> u32 {
    let mut result: u32 = 0;
    let mut last_number: String = String::new();
    let mut last: bool = false;
    let mut index: usize = 1;
    let numbers: Vec<&str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut matches: Vec<usize> = vec![];
    let mut is_match: bool = false;

    let mut current = 0;
    let line_len = line.len();

    while current < line_len {
        let c = line.chars().nth(current).unwrap(); // caracter actual
        if c.is_numeric() {
            // si es un numero
            last_number = c.to_string();

            if !last {
                result += 10 * last_number.parse::<u32>().unwrap();
                last = true;
            }

            current += 1;
            index = 1;
            matches.clear();
        } else {
            if matches.len() > 0 {
                let prev_matches = matches.clone();
                matches.clear();
                for m in prev_matches.iter() {
                    let number = numbers.get(*m).unwrap();
                    if number.chars().nth(index).unwrap() == c {
                        if index + 1 == number.len() {
                            last_number = (m + 1).to_string();

                            if !last {
                                result += 10 * last_number.parse::<u32>().unwrap();
                                last = true;
                            }

                            index = 1; // reinicio el indice
                            matches.clear();
                            break;
                        } else {
                            matches.push(*m);
                            is_match = true;
                        }
                    }
                }

                if is_match {
                    index += 1;
                    is_match = false;
                    current += 1;
                } else {
                    current = if (current - index + 1) < 0 {
                        0
                    } else {
                        current - index + 1
                    };
                    index = 1;
                }
            } else {
                for (i, n) in numbers.iter().enumerate() {
                    if n.chars().nth(0).unwrap() == c {
                        matches.push(i);
                    }
                }
                current += 1;
            }
        }
    }

    result += last_number.parse::<u32>().unwrap();

    result
}
