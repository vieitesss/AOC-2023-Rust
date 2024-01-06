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

pub fn solve_day(year: u16, day: u8, input: u8) -> std::io::Result<()> {
    let mut input_str = String::new();

    match input {
        0 => File::open(format!("data/aoc{}/day{}/input.txt", year, day).as_str())?
            .read_to_string(&mut input_str)?,
        1 => File::open(format!("data/aoc{}/day{}/example1.txt", year, day).as_str())?
            .read_to_string(&mut input_str)?,
        2 => File::open(format!("data/aoc{}/day{}/example2.txt", year, day).as_str())?
            .read_to_string(&mut input_str)?,
        _ => panic!("Invalid input"),
    };

    match day {
        3 => day3::Day3::solve(input_str.as_str()),
        4 => day4::Day4::solve(input_str.as_str()),
        5 => day5::Day5::solve(input_str.as_str()),
        6 => day6::Day6::solve(input_str.as_str()),
        7 => day7::Day7::solve(input_str.as_str()),
        8 => day8::Day8::solve(input_str.as_str()),
        9 => day9::Day9::solve(input_str.as_str()),
        10 => day10::Day10::solve(input_str.as_str()),
        11 => day11::Day11::solve(input_str.as_str()),
        12 => day12::Day12::solve(input_str.as_str()),
        13 => day13::Day13::solve(input_str.as_str()),
        14 => day14::Day14::solve(input_str.as_str()),
        15 => day15::Day15::solve(input_str.as_str()),
        16 => day16::Day16::solve(input_str.as_str()),
        17 => day17::Day17::solve(input_str.as_str()),
        _ => (
            String::from("Not implemented"),
            String::from("Not implemented"),
        ),
    };

    Ok(())
}
