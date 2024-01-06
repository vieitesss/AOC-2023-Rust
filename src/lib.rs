#![feature(lazy_cell)]
#![feature(extract_if)]

pub mod aoc2023;

use std::fs::File;
use std::io::Read;

// use aoc2023::day1;
// use aoc2023::day2;
use aoc2023::day10;
use aoc2023::day11;
use aoc2023::day12;
use aoc2023::day13;
use aoc2023::day14;
use aoc2023::day15;
use aoc2023::day16;
use aoc2023::day17;
use aoc2023::day3;
use aoc2023::day4;
use aoc2023::day5;
use aoc2023::day6;
use aoc2023::day7;
use aoc2023::day8;
use aoc2023::day9;

pub trait Solution {
    type ParsedInput;

    fn parse_input(input_lines: &str) -> Self::ParsedInput;
    fn part_1(parsed_input: &Self::ParsedInput) -> String;
    fn part_2(parsed_input: Self::ParsedInput) -> String;

    fn solve_part_1(input_lines: &str) -> String {
        Self::part_1(&Self::parse_input(input_lines))
    }
    fn solve_part_2(input_lines: &str) -> String {
        Self::part_2(Self::parse_input(input_lines))
    }
    fn solve(input_lines: &str) -> (String, String) {
        use std::time::Instant;
        let now = Instant::now();
        let input = Self::parse_input(input_lines);
        let parse_time = now.elapsed().as_micros();
        println!(
            "Parsing: {:2}.{:03} ms",
            parse_time / 1000,
            parse_time & 1000
        );

        let now = Instant::now();
        let p1 = Self::part_1(&input);
        let part1_time = now.elapsed().as_micros();
        println!(
            "Part 1: {:3}.{:03} ms -> {}",
            part1_time / 1000,
            part1_time & 1000,
            p1
        );

        let now = Instant::now();
        let p2 = Self::part_2(input);
        let part2_time = now.elapsed().as_micros();
        println!(
            "Part 2: {:3}.{:03} ms -> {}",
            part2_time / 1000,
            part2_time & 1000,
            p2
        );
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
                _ => (String::from("Invalid..."), String::from("...input")),
            };
        }
        4 => {
            match input {
                0 => day4::Day4::solve(include_str!("../data/day4/input.txt")),
                1 => day4::Day4::solve(include_str!("../data/day4/example1.txt")),
                2 => day4::Day4::solve(include_str!("../data/day4/example2.txt")),
                _ => (String::from("Invalid..."), String::from("...input")),
            };
        }
        5 => {
            match input {
                0 => day5::Day5::solve(include_str!("../data/day5/input.txt")),
                1 => day5::Day5::solve(include_str!("../data/day5/example1.txt")),
                2 => day5::Day5::solve(include_str!("../data/day5/example1.txt")),
                _ => (String::from("Invalid..."), String::from("...input")),
            };
        }
        6 => {
            match input {
                0 => day6::Day6::solve(include_str!("../data/day6/input.txt")),
                1 => day6::Day6::solve(include_str!("../data/day6/example1.txt")),
                2 => day6::Day6::solve(include_str!("../data/day6/example1.txt")),
                _ => (String::from("Invalid..."), String::from("...input")),
            };
        }
        7 => {
            match input {
                0 => day7::Day7::solve(include_str!("../data/day7/input.txt")),
                1 => day7::Day7::solve(include_str!("../data/day7/example1.txt")),
                2 => day7::Day7::solve(include_str!("../data/day7/example1.txt")),
                _ => (String::from("Invalid..."), String::from("...input")),
            };
        }
        8 => {
            match input {
                0 => day8::Day8::solve(include_str!("../data/day8/input.txt")),
                1 => day8::Day8::solve(include_str!("../data/day8/example1.txt")),
                2 => day8::Day8::solve(include_str!("../data/day8/example2.txt")),
                _ => (String::from("Invalid..."), String::from("...input")),
            };
        }
        9 => {
            match input {
                0 => day9::Day9::solve(include_str!("../data/day9/input.txt")),
                1 => day9::Day9::solve(include_str!("../data/day9/example1.txt")),
                2 => day9::Day9::solve(include_str!("../data/day9/example1.txt")),
                _ => (String::from("Invalid..."), String::from("...input")),
            };
        }
        10 => {
            match input {
                0 => day10::Day10::solve(include_str!("../data/day10/input.txt")),
                1 => day10::Day10::solve(include_str!("../data/day10/example1.txt")),
                2 => day10::Day10::solve(include_str!("../data/day10/example2.txt")),
                _ => (String::from("Invalid..."), String::from("...input")),
            };
        }
        11 => {
            match input {
                0 => day11::Day11::solve(include_str!("../data/day11/input.txt")),
                1 => day11::Day11::solve(include_str!("../data/day11/example1.txt")),
                // 2 => day11::Day11::solve(include_str!("../data/day11/example2.txt")),
                _ => (String::from("Invalid..."), String::from("...input")),
            };
        }
        12 => {
            match input {
                0 => day12::Day12::solve(include_str!("../data/day12/input.txt")),
                1 => day12::Day12::solve(include_str!("../data/day12/example1.txt")),
                2 => day12::Day12::solve(include_str!("../data/day12/example2.txt")),
                _ => (String::from("Invalid..."), String::from("...input")),
            };
        }
        13 => {
            match input {
                0 => day13::Day13::solve(include_str!("../data/day13/input.txt")),
                1 => day13::Day13::solve(include_str!("../data/day13/example1.txt")),
                2 => day13::Day13::solve(include_str!("../data/day13/example2.txt")),
                _ => (String::from("Invalid..."), String::from("...input")),
            };
        }
        14 => {
            match input {
                0 => day14::Day14::solve(include_str!("../data/day14/input.txt")),
                1 => day14::Day14::solve(include_str!("../data/day14/example1.txt")),
                2 => day14::Day14::solve(include_str!("../data/day14/example2.txt")),
                _ => (String::from("Invalid..."), String::from("...input")),
            };
        }
        15 => {
            match input {
                0 => day15::Day15::solve(include_str!("../data/day15/input.txt")),
                1 => day15::Day15::solve(include_str!("../data/day15/example1.txt")),
                _ => (String::from("Invalid..."), String::from("...input")),
            };
        }
        16 => {
            match input {
                0 => day16::Day16::solve(include_str!("../data/day16/input.txt")),
                1 => day16::Day16::solve(include_str!("../data/day16/example1.txt")),
                _ => (String::from("Invalid..."), String::from("...input")),
            };
        }
        17 => {
            match input {
                0 => day17::Day17::solve(include_str!("../data/day17/input.txt")),
                1 => day17::Day17::solve(include_str!("../data/day17/example1.txt")),
                // 2 => day17::Day17::solve(include_str!("../data/day17/example2.txt")),
                _ => (String::from("Invalid..."), String::from("...input")),
            };
        }
        _ => (),
    }
}
