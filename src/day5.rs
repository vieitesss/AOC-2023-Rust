#![allow(dead_code)]
#![allow(unused_variables)]

use std::usize::MAX;

use crate::Solution;

pub struct Day5;

#[derive(Debug)]
pub struct Almanac {
    initial_numbers: Vec<usize>,
    maps: Vec<Map>,
}

impl Almanac {
    fn new() -> Self {
        Self {
            initial_numbers: vec![],
            maps: vec![],
        }
    }

    fn get_lowest_location(&self) -> String {
        self.initial_numbers
            .iter()
            .fold(MAX, |cur, n| {
                let result = self
                    .maps
                    .iter()
                    .fold(n.clone(), |cur, v| v.get_mapped_value(cur));

                if result < cur {
                    return result;
                }

                cur
            })
            .to_string()
    }
}

#[derive(Debug, Clone)]
struct Map {
    lines: Vec<Line>,
}

impl Map {
    fn new() -> Self {
        Self { lines: vec![] }
    }

    fn get_mapped_value(&self, number: usize) -> usize {
        if let Some(tuple) = self.index_of_value(number) {
            self.lines[tuple.0].destination + tuple.1
        } else {
            number
        }
    }

    fn index_of_value(&self, number: usize) -> Option<(usize, usize)> {
        for (i, l) in self.lines.iter().enumerate() {
            if let Some(index) = l.index_of_value(number) {
                return Some((i, index));
            }
        }

        None
    }
}

#[derive(Debug, Clone)]
struct Line {
    destination: usize,
    source: usize,
    range: usize,
}

impl Line {
    fn index_of_value(&self, number: usize) -> Option<usize> {
        if self.source < number && self.source + self.range > number {
            return Some(number - self.source);
        }

        None
    }
}

impl Solution for Day5 {
    type ParsedInput = Almanac;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let mut almanac = Almanac::new();
        let mut mapping = false;
        let mut map = Map::new();
        for line in input_lines.lines() {
            if line.is_empty() || line.contains("map") {
                if !mapping {
                    mapping = true;
                } else if map.lines.len() > 0 {
                    almanac.maps.push(map.clone());
                    map = Map::new();
                }
                continue;
            }

            if line.starts_with("seeds") {
                almanac.initial_numbers = line
                    .split(':')
                    .nth(1)
                    .unwrap()
                    .split_whitespace()
                    .map(|i| i.parse().unwrap())
                    .collect();
                continue;
            }

            // parse each line of numbers
            let triplet: Vec<usize> = line
                .split_whitespace()
                .map(|i| i.parse().unwrap())
                .collect();

            let destination = triplet[0];
            let source = triplet[1];
            let range = triplet[2];

            let l = Line {
                destination,
                source,
                range,
            };
            map.lines.push(l);
        }

        almanac.maps.push(map.clone());

        almanac
    }

    fn part_1(almanac: &mut Self::ParsedInput) -> String {
        almanac.get_lowest_location()
    }

    fn part_2(parsed_input: &mut Self::ParsedInput) -> String {
        "".to_string()
    }
}

