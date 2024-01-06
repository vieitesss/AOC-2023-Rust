use std::usize::MAX;

use crate::Solution;

pub struct Day5;

#[derive(Debug)]
pub struct Almanac {
    initial_numbers: Vec<usize>,
    pairs: Vec<(usize, usize)>,
    maps: Vec<Map>,
}

impl Almanac {
    fn new() -> Self {
        Self {
            initial_numbers: vec![],
            pairs: vec![],
            maps: vec![],
        }
    }

    fn set_pairs(&self) -> Vec<(usize, usize)> {
        (0..self.initial_numbers.len())
            .step_by(2)
            .map(|i| (self.initial_numbers[i], self.initial_numbers[i + 1]))
            .collect::<Vec<(usize, usize)>>()
    }

    fn get_lowest_location(&self) -> String {
        self.initial_numbers
            .iter()
            .fold(MAX, |cur, n| {
                let result = self.get_location(*n);

                if result < cur {
                    return result;
                }

                cur
            })
            .to_string()
    }

    fn get_lowest_location_from_pairs(&self) -> String {
        self.pairs
            .iter()
            .fold(MAX, |min, pair| {
                let min_of_pair = self.get_min_location_of_pair(*pair);

                if min_of_pair < min {
                    return min_of_pair;
                }

                min
            })
            .to_string()
    }

    fn get_min_location_of_pair(&self, pair: (usize, usize)) -> usize {
        let mut min = MAX;
        let mut current_value = pair.0;

        while current_value < pair.0 + pair.1 {
            let result = self.get_location(current_value);
            let distance = self.get_distance_to_next_range(current_value);

            min = if result < min { result } else { min };

            current_value += if distance == MAX { 1 } else { distance };
        }

        min
    }

    fn get_location(&self, number: usize) -> usize {
        self.maps.iter().fold(number, |n, m| m.get_mapped(n))
    }

    fn get_distance_to_next_range(&self, number: usize) -> usize {
        self.maps
            .iter()
            .fold((number, MAX), |(n, min), m| {
                let distance = m.get_distance_to_next_range(n);
                let mapped = m.get_mapped(n);
                if distance < min {
                    return (mapped, distance);
                }

                (mapped, min)
            })
            .1
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

    fn get_mapped(&self, number: usize) -> usize {
        if let Some((line, index)) = self.get_line_and_index(number) {
            self.lines[line].destination + index
        } else {
            number
        }
    }

    fn get_line_and_index(&self, number: usize) -> Option<(usize, usize)> {
        for (i, l) in self.lines.iter().enumerate() {
            if let Some(index) = l.index_of_value(number) {
                return Some((i, index));
            }
        }

        None
    }

    fn get_distance_to_next_range(&self, number: usize) -> usize {
        if let Some((line, _)) = self.get_line_and_index(number) {
            self.lines[line].source + self.lines[line].range - number
        } else {
            self.lines.iter().fold(MAX, |cur, l| {
                if l.source < number {
                    return cur;
                }

                if l.source - number < cur {
                    return l.source - number;
                }

                cur
            })
        }
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
        if self.source <= number && self.source + self.range > number {
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

        almanac.pairs = almanac.set_pairs();

        almanac
    }

    fn part_1(almanac: &Self::ParsedInput) -> String {
        almanac.get_lowest_location()
    }

    fn part_2(almanac: Self::ParsedInput) -> String {
        almanac.get_lowest_location_from_pairs()
    }
}
