#![allow(dead_code)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT_PATH: &str = "data/day3/input.txt";
const EXAMPLE1_PATH: &str = "data/day3/example1.txt";
const _EXAMPLE2_PATH: &str = "data/day3/example2.txt";

pub fn resolve() {
    println!("Part 1: {}", part_1());
    // println!("Part 2: {}", part2());
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn part_1() -> u32 {
    let file = File::open(INPUT_PATH).expect("no such file");
    let buf = BufReader::new(file);
    let matrix: Vec<String> = buf
        .lines()
        .map(|l| l.expect("could not parse line"))
        .collect();

    get_sum(&matrix)
}

fn get_sum(matrix: &[String]) -> u32 {
    let mut sum: u32 = 0;

    let mut number: Vec<Point> = vec![];
    for (x, line) in matrix.iter().enumerate() {
        for (y, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                number.push(Point { x, y });
            } else if number.len() > 0 {
                if is_part_number(number.first().unwrap(), number.last().unwrap(), &matrix) {
                    sum += compose_number(&number, &matrix);
                }
                number.clear();
            }
        }
    }

    sum
}

fn is_part_number(first: &Point, last: &Point, matrix: &[String]) -> bool {
    let up = if first.x > 0 { first.x - 1 } else { first.x };
    let down = if first.x < matrix.len() - 1 {
        first.x + 1
    } else {
        first.x
    };
    let left = if first.y > 0 { first.y - 1 } else { first.y };
    let right = if last.y < matrix[0].len() - 1 {
        last.y + 1
    } else {
        last.y
    };

    for x in up..down + 1 {
        for y in left..right + 1 {
            if x == first.x && y >= first.y && y <= last.y {
                continue;
            }
            let c = matrix[x].chars().nth(y).unwrap();
            if !c.is_digit(10) && c != '.' {
                return true;
            }
        }
    }

    false
}

fn compose_number(array_number: &[Point], matrix: &[String]) -> u32 {
    let mut number: String = String::new();

    for p in array_number.iter() {
        number.push(matrix[p.x].chars().nth(p.y).unwrap());
    }

    number.parse::<u32>().unwrap()
}

fn part_2() -> u32 {
    todo!();
}
