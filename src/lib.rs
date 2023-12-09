pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;


pub trait Solution {
    type ParsedInput;

    fn parse_input(input_lines: &str) -> Self::ParsedInput;
    fn part_1(parsed_input: &mut Self::ParsedInput) -> String;
    fn part_2(parsed_input: &mut Self::ParsedInput) -> String;
    fn solve_part_1(input_lines: &str) -> String {
        Self::part_1(&mut Self::parse_input(input_lines))
    }
    fn solve_part_2(input_lines: &str) -> String {
        Self::part_2(&mut Self::parse_input(input_lines))
    }
    fn solve(input_lines: &str) -> (String, String) {
        let mut input = Self::parse_input(input_lines);
        let p1 = Self::part_1(&mut input);
        let p2 = Self::part_2(&mut input);
        println!("Part 1: {}\nPart 2: {}", p1, p2);
        (p1, p2)
    }
}

pub fn solve_day(day: u8, input: u8) {
    match day {
        3 => {
            match input {
                0 => day3::Day3::solve(include_str!("../data/day3/input.txt")),
                1 => day3::Day3::solve(include_str!("../data/day3/example1.txt")),
                2 => day3::Day3::solve(include_str!("../data/day3/example2.txt")),
                _ => (String::from("Invalid..."), String::from("...input"))
            };
        },
        4 => {
            match input {
                0 => day4::Day4::solve(include_str!("../data/day4/input.txt")),
                1 => day4::Day4::solve(include_str!("../data/day4/example1.txt")),
                2 => day4::Day4::solve(include_str!("../data/day4/example2.txt")),
                _ => (String::from("Invalid..."), String::from("...input"))
            };
        },
        _ => ()
    }
}
