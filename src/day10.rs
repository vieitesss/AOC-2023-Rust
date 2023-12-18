#![allow(unused_variables)]
#![allow(dead_code)]

use std::fmt::Display;

use phf::{phf_map, Map};

use crate::Solution;

const DIRS: [[i32; 2]; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];

static SYMBOLS: Map<char, &'static [u8; 4]> = phf_map! {
    '|' => &[1, 0, 1, 0],
    '-' => &[0, 1, 0, 1],
    'L' => &[1, 1, 0, 0],
    'J' => &[1, 0, 0, 1],
    '7' => &[0, 0, 1, 1],
    'F' => &[0, 1, 1, 0],
    '.' => &[0, 0, 0, 0],
    'S' => &[0, 0, 0, 0],
};

#[derive(Debug, Clone)]
struct Point(i32, i32);

#[derive(Debug)]
pub struct Board {
    init: Point,
    matrix: Vec<Vec<Vec<u8>>>,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, line) in self.matrix.iter().enumerate() {
            println!("{:?} ", line);
        }

        Ok(())
    }
}

impl Board {
    fn update_point(&mut self, point: Point, dir: usize) {
        self.matrix
            .get_mut(point.0 as usize)
            .unwrap()
            .get_mut(point.1 as usize)
            .unwrap()[dir] = 0;
    }

    fn get_init_points(&mut self) -> Vec<Point> {
        DIRS.iter()
            .enumerate()
            .map(|(i, [x, y])| {
                let p = Point(self.init.0.clone() + x, self.init.1 + y);
                let dir = (i + 2) % 4;
                if self.is_in_bounds(p.clone()) && self.is_possible_move(p.clone(), dir) {
                    self.update_point(p.clone(), dir);
                    return Some(p);
                }
                None
            })
            .filter(|p| p.is_some())
            .map(|p| p.unwrap())
            .collect()
    }

    fn is_possible_move(&self, point: Point, dir: usize) -> bool {
        *self
            .matrix
            .get(point.0 as usize)
            .unwrap()
            .get(point.1 as usize)
            .unwrap()
            .get(dir)
            .unwrap()
            == 1
    }

    fn is_in_bounds(&self, p: Point) -> bool {
        return p.0 >= 0
            && p.0 < self.matrix.len() as i32
            && p.1 >= 0
            && p.1 < self.matrix[0].len() as i32;
    }

    fn get_next_point(&mut self, point: Point) -> Point {
        let pos = self
            .matrix
            .get(point.0 as usize)
            .unwrap()
            .get(point.1 as usize)
            .unwrap()
            .iter()
            .position(|&pos| pos == 1)
            .unwrap();

        let p = Point(point.0 + DIRS[pos][0], point.1 + DIRS[pos][1]);
        self.update_point(p.clone(), (pos + 2) % 4);
        p
    }
}

pub struct Day10;

impl Solution for Day10 {
    type ParsedInput = Board;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let mut init = Point(0, 0);
        let matrix = input_lines
            .lines()
            .enumerate()
            .map(|(x, l)| {
                l.chars()
                    .enumerate()
                    .map(|(y, c)| {
                        if c == 'S' {
                            init = Point(x as i32, y as i32);
                        }
                        SYMBOLS.get(&c).unwrap().to_vec()
                    })
                    .collect()
            })
            .collect();

        Board { init, matrix }
    }

    fn part_1(parsed_input: &mut Self::ParsedInput) -> String {
        let mut points = parsed_input.get_init_points();

        let mut distance = 1;
        while points[0].0 != points[1].0 || points[0].1 != points[1].1 {
            points = points
                .iter()
                .map(|p| parsed_input.get_next_point(p.clone()))
                .collect();

            distance += 1;
        }

        distance.to_string()
    }

    fn part_2(parsed_input: &mut Self::ParsedInput) -> String {
        "".to_string()
    }
}
