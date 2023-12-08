use crate::Solution;
use std::cmp::min;

pub struct Day3;

#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub struct Grid(Vec<Vec<char>>);

impl Grid {
    fn get_lines(&self) -> usize {
        self.0.len()
    }

    fn get_columns(&self) -> usize {
        self.0[0].len()
    }

    fn get_top_bound(&self, point: &Point) -> usize {
        if point.x > 0 {
            point.x - 1
        } else {
            0
        }
    }

    fn get_bottom_bound(&self, point: &Point) -> usize {
        min(point.x + 1, self.get_lines() - 1)
    }

    fn get_left_bound(&self, point: &Point) -> usize {
        if point.y > 0 {
            point.y - 1
        } else {
            0
        }
    }

    fn get_right_bound(&self, point: &Point) -> usize {
        min(point.y + 1, self.get_columns() - 1)
    }

    // (top, bottom, left, right)
    fn get_bounds(&self, first: &Point, last: &Point) -> (usize, usize, usize, usize) {
        (
            self.get_top_bound(first),
            self.get_bottom_bound(first),
            self.get_left_bound(first),
            self.get_right_bound(last),
        )
    }

    fn get_part_numbers(&self) -> Vec<u32> {
        let mut part_numbers: Vec<u32> = vec![];

        let mut number: Vec<Point> = vec![];
        for (x, line) in self.0.iter().enumerate() {
            for (y, char) in line.iter().enumerate() {
                if char.is_digit(10) {
                    number.push(Point { x, y });
                } else if number.clone().len() > 0 {
                    if self.is_part_number(&number) {
                        part_numbers.push(self.compose_number(&number));
                    }
                    number.clear();
                }
            }
        }

        part_numbers
    }

    fn is_part_number(&self, number: &Vec<Point>) -> bool {
        let first = number.first().unwrap();
        let last = number.last().unwrap();
        let (up, down, left, right) = self.get_bounds(first, last);

        for x in up..=down {
            for y in left..=right {
                if x == first.x && y >= first.y && y <= last.y {
                    continue;
                }
                let c = self.0[x][y];
                if !c.is_digit(10) && c != '.' {
                    return true;
                }
            }
        }

        false
    }

    fn compose_number(&self, number_points: &Vec<Point>) -> u32 {
        number_points
            .iter()
            .map(|p| self.0[p.x][p.y])
            .fold("".to_string(), |cur, val| format!("{}{}", cur, val))
            .parse::<u32>()
            .unwrap()
    }
}

impl Solution for Day3 {
    type ParsedInput = Grid;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let data = input_lines.lines().map(|l| l.chars().collect()).collect();

        Grid(data)
    }

    fn part_1(parsed_input: &mut Self::ParsedInput) -> String {
        format!("{}", parsed_input.get_part_numbers().iter().sum::<u32>())
    }

    fn part_2(_parsed_input: &mut Self::ParsedInput) -> String {
        // TODO: implement part 1
        "".to_string()
    }
}
