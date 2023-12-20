use std::{cell::LazyCell, cmp::min, collections::HashMap, fmt::Display};

use crate::Solution;

const DIRS: [[i32; 2]; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];

const SYMBOLS: LazyCell<HashMap<char, &'static [u8; 4]>> = LazyCell::new(|| {
    let mut h = HashMap::new();
    h.insert('|', &[1, 0, 1, 0]);
    h.insert('-', &[0, 1, 0, 1]);
    h.insert('L', &[1, 1, 0, 0]);
    h.insert('J', &[1, 0, 0, 1]);
    h.insert('7', &[0, 0, 1, 1]);
    h.insert('F', &[0, 1, 1, 0]);
    h.insert('.', &[0, 0, 0, 0]);
    h.insert('S', &[0, 0, 0, 0]);
    h
});

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point(i32, i32);

#[derive(Debug, Clone)]
pub struct Board {
    init: Point,
    matrix: Vec<Vec<Vec<u8>>>,
    road_points: Vec<Point>,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.matrix.iter() {
            writeln!(f, "{:?}", line)?;
        }

        Ok(())
    }
}

impl Board {
    fn update_point(&mut self, point: Point, dir: usize, val: u8) {
        self.matrix[point.0 as usize][point.1 as usize][dir] = val;
    }

    fn update_init_dirs(&mut self) {
        DIRS.iter().enumerate().for_each(|(i, [x, y])| {
            let p = Point(self.init.0 + x, self.init.1 + y);
            let dir = (i + 2) % 4;
            if self.is_in_bounds(&p) && self.is_possible_move(&p, dir) {
                self.get_point_dirs(&self.init)[i] = 1;
                self.update_point(self.init, i, 1);
            }
        });
    }

    fn is_possible_move(&self, point: &Point, dir: usize) -> bool {
        *self.get_point_dirs(point).get(dir).unwrap() == 1
    }

    fn is_in_bounds(&self, p: &Point) -> bool {
        return p.0 >= 0
            && p.0 < self.matrix.len() as i32
            && p.1 >= 0
            && p.1 < self.matrix[0].len() as i32;
    }

    fn get_point_dirs(&self, point: &Point) -> Vec<u8> {
        self.matrix
            .get(point.0 as usize)
            .unwrap()
            .get(point.1 as usize)
            .unwrap()
            .to_vec()
    }

    fn is_in_road(&self, point: &Point) -> bool {
        self.road_points.iter().any(|p| p.eq(point))
    }

    fn inside_in_line(&self, row: usize, column: usize, up: i32, down: i32, sum: i32) -> i32 {
        if column == self.matrix[0].len() {
            return sum;
        }

        let point = Point(row as i32, column as i32);
        if self.is_in_road(&point) {
            let point_dirs = self.get_point_dirs(&point);
            return self.inside_in_line(
                row,
                column + 1,
                up + point_dirs[0] as i32,
                down + point_dirs[2] as i32,
                sum,
            );
        } else if min(up, down) % 2 == 1 {
            return self.inside_in_line(row, column + 1, up, down, sum + 1);
        }

        self.inside_in_line(row, column + 1, up, down, sum)
    }

    fn find_road(&mut self, point: &Point, dir: usize, len: usize) -> usize {
        if point.eq(&self.init) && len > 0 {
            return len;
        }
        self.road_points.push(*point);

        let next_point = Point(point.0 + DIRS[dir][0], point.1 + DIRS[dir][1]);
        let next_point_dir = self
            .get_point_dirs(&next_point)
            .iter()
            .enumerate()
            .position(|(i, d)| i != (dir + 2) % 4 && *d == 1)
            .unwrap();

        self.find_road(&next_point, next_point_dir, len + 1)
    }
}

pub struct Day10;

impl Solution for Day10 {
    type ParsedInput = Board;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let mut init = Point(0, 0);
        let mut tails = vec![];
        let matrix = input_lines
            .lines()
            .enumerate()
            .map(|(x, l)| {
                l.chars()
                    .enumerate()
                    .map(|(y, c)| {
                        if c == '.' {
                            tails.push(Point(x as i32, y as i32));
                        }
                        if c == 'S' {
                            init = Point(x as i32, y as i32);
                        }
                        SYMBOLS.get(&c).unwrap().to_vec()
                    })
                    .collect()
            })
            .collect();

        Board {
            init,
            matrix,
            road_points: vec![],
        }
    }

    fn part_1(parsed_input: &Self::ParsedInput) -> String {
        let mut part1 = parsed_input.clone();
        part1.update_init_dirs();
        let dir = part1
            .get_point_dirs(&parsed_input.init)
            .iter()
            .position(|d| *d == 1)
            .unwrap();
        let road_len = part1.find_road(&parsed_input.init, dir, 0);

        (road_len / 2).to_string()
    }

    fn part_2(parsed_input: Self::ParsedInput) -> String {
        let mut part2 = parsed_input.clone();
        part2.update_init_dirs();
        let dir = part2
            .get_point_dirs(&parsed_input.init)
            .iter()
            .position(|d| *d == 1)
            .unwrap();
        let _ = part2.find_road(&parsed_input.init, dir, 0);

        let total = part2
            .matrix
            .iter()
            .enumerate()
            .fold(0, |cur, (x, _)| cur + part2.inside_in_line(x, 0, 0, 0, 0));

        total.to_string()
    }
}
