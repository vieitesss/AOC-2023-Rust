use std::fs::read_to_string;

const INPUT_PATH: &str = "data/day2/input.txt";
const _EXAMPLE1_PATH: &str = "data/day2/example1.txt";
const _EXAMPLE2_PATH: &str = "data/day2/example2.txt";

#[derive(Debug)]
struct Hand {
    green_balls: u32,
    blue_balls: u32,
    red_balls: u32,
}

impl Hand {
    pub fn new() -> Self {
        Self {
            green_balls: 0,
            blue_balls: 0,
            red_balls: 0,
        }
    }

    pub const fn default() -> Self {
        Self {
            green_balls: 13,
            blue_balls: 14,
            red_balls: 12,
        }
    }

    pub fn set_green_balls(&mut self, number: u32) {
        self.green_balls = number;
    }

    pub fn set_red_balls(&mut self, number: u32) {
        self.red_balls = number;
    }

    pub fn set_blue_balls(&mut self, number: u32) {
        self.blue_balls = number;
    }

    pub fn multiply(&mut self) -> u32 {
        (self.green_balls % u32::MAX) * (self.blue_balls % u32::MAX) * (self.red_balls % u32::MAX)
    }
}

const DEFAULT_HAND: Hand = Hand::default();

pub fn resolve() {
    println!("Hello day 2");

    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
}

fn get_n_color_pairs(line: &str) -> Vec<&str> {
    let all_hands: &str = line.split(":").collect::<Vec<&str>>()[1].trim();
    let n_color_pairs: Vec<&str> = all_hands
        .split(&[',', ';'])
        .map(|item| item.trim())
        .collect();

    n_color_pairs
}

fn get_n_color_tuple(n_color_pair: &str) -> (u32, &str) {
    let pair: Vec<&str> = n_color_pair.split(" ").collect();

    (pair[0].parse::<u32>().unwrap(), pair[1])
}

fn part_1() -> u32 {
    let mut sum: u32 = 0;

    for (i, line) in read_to_string(INPUT_PATH).unwrap().lines().enumerate() {
        if is_valid_game(line) {
            sum += i as u32 + 1;
        }
    }

    sum
}

fn is_valid_game(line: &str) -> bool {
    let mut hand = Hand::new();
    let n_color_pairs = get_n_color_pairs(line);

    for n_color_pair in n_color_pairs.iter() {
        let (number, color) = get_n_color_tuple(n_color_pair);
        match color {
            "red" => hand.set_red_balls(number),
            "blue" => hand.set_blue_balls(number),
            "green" => hand.set_green_balls(number),
            _ => (),
        }
        if DEFAULT_HAND.green_balls < hand.green_balls
            || DEFAULT_HAND.blue_balls < hand.blue_balls
            || DEFAULT_HAND.red_balls < hand.red_balls
        {
            return false;
        }
    }

    true
}

fn part_2() -> u32 {
    let mut sum: u32 = 0;

    for line in read_to_string(INPUT_PATH).unwrap().lines() {
        sum += get_power(line);
    }

    sum
}

fn get_power(line: &str) -> u32 {
    let mut hand = Hand::new();
    let n_color_pairs = get_n_color_pairs(line);

    for n_color_pair in n_color_pairs.iter() {
        let (number, color) = get_n_color_tuple(n_color_pair);
        match color {
            "red" => {
                if number > hand.red_balls {
                    hand.set_red_balls(number)
                }
            }
            "blue" => {
                if number > hand.blue_balls {
                    hand.set_blue_balls(number)
                }
            }
            "green" => {
                if number > hand.green_balls {
                    hand.set_green_balls(number)
                }
            }
            _ => (),
        }
    }

    hand.multiply()
}
