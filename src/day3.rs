use crate::Solution;
use std::cmp::{max, min};

pub struct Day3;

#[derive(Debug, Clone)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone)]
struct Gear {
    n1: usize,
    n2: usize,
}

#[derive(Debug)]
pub struct Grid(Vec<Vec<char>>);

impl Grid {
    fn get_lines(&self) -> isize {
        self.0.len() as isize
    }

    fn get_columns(&self) -> isize {
        self.0[0].len() as isize
    }

    fn get_top_bound(&self, point: &Point) -> usize {
        max(point.x - 1, 0) as usize
    }

    fn get_bottom_bound(&self, point: &Point) -> usize {
        min(point.x + 1, self.get_lines() - 1) as usize
    }

    fn get_left_bound(&self, point: &Point) -> usize {
        max(point.y - 1, 0) as usize
    }

    fn get_right_bound(&self, point: &Point) -> usize {
        min(point.y + 1, self.get_columns() - 1) as usize
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

    fn get_part_numbers(&self) -> Vec<usize> {
        let mut part_numbers: Vec<usize> = vec![];

        let mut number: Vec<Point> = vec![];
        for (x, line) in self.0.iter().enumerate() {
            for (y, char) in line.iter().enumerate() {
                if char.is_digit(10) {
                    number.push(Point {
                        x: x as isize,
                        y: y as isize,
                    });
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

    fn is_part_number(&self, number: &[Point]) -> bool {
        let first = number.first().unwrap();
        let last = number.last().unwrap();
        let (up, down, left, right) = self.get_bounds(first, last);

        for x in up..=down {
            for y in left..=right {
                if x == first.x as usize && y >= first.y as usize && y <= last.y as usize {
                    continue;
                }
                let c = self.0[x as usize][y as usize];
                if !c.is_digit(10) && c != '.' {
                    return true;
                }
            }
        }

        false
    }

    fn compose_number(&self, number_points: &[Point]) -> usize {
        number_points
            .iter()
            .map(|p| self.0[p.x as usize][p.y as usize])
            .fold("".to_string(), |cur, val| format!("{}{}", cur, val))
            .parse()
            .unwrap()
    }

    fn get_ratios(&self) -> Vec<usize> {
        let gears = self.get_gears();

        gears.iter().map(|g| g.n1 * g.n2).collect()
    }

    fn get_gears(&self) -> Vec<Gear> {
        let stars = self.get_stars();

        stars
            .iter()
            .map(|p| self.to_gear(p))
            .filter(|s| s.len() == 2)
            .map(|g| Gear { n1: g[0], n2: g[1] })
            .collect()
    }

    fn to_gear(&self, star: &Point) -> Vec<usize> {
        let (top, bottom, left, right) = self.get_bounds(star, star);

        let mut part_numbers: Vec<usize> = vec![];
        let mut search_symbol = false;

        let mut last_point_n = Point { x: -1, y: -1 };

        for x in top..=bottom {
            for y in left..=right {
                if self.0[x][y].is_digit(10) {
                    last_point_n = Point {
                        x: x as isize,
                        y: y as isize,
                    };
                    if !search_symbol {
                        search_symbol = true;
                    }
                } else if search_symbol {
                    let n = self.get_number_in_point(&last_point_n);
                    part_numbers.push(n);
                    search_symbol = false;
                }
            }
            if search_symbol {
                let n = self.get_number_in_point(&last_point_n);
                part_numbers.push(n);
                search_symbol = false;
            }
        }

        if search_symbol {
            let n = self.get_number_in_point(&last_point_n);
            part_numbers.push(n);
        }

        part_numbers
    }

    fn get_number_in_point(&self, point: &Point) -> usize {
        let x = point.x as usize;
        let mut current_y = point.y as usize;

        loop {
            if current_y == 0 {
                break;
            }

            if self.0[x][current_y - 1].is_digit(10) {
                current_y -= 1;
            } else {
                break;
            }
        }

        let mut number: Vec<Point> = vec![Point {
            x: x as isize,
            y: current_y as isize,
        }];

        loop {
            current_y += 1;

            if current_y == self.get_columns() as usize {
                break;
            }

            if !self.0[x][current_y].is_digit(10) {
                break;
            }

            number.push(Point {
                x: x as isize,
                y: current_y as isize,
            })
        }

        self.compose_number(&number)
    }

    fn get_stars(&self) -> Vec<Point> {
        let mut stars: Vec<Point> = vec![];

        for (x, line) in self.0.iter().enumerate() {
            for (y, c) in line.iter().enumerate() {
                if *c == '*' {
                    stars.push(Point {
                        x: x as isize,
                        y: y as isize,
                    })
                }
            }
        }

        stars
    }
}

impl Solution for Day3 {
    type ParsedInput = Grid;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let data = input_lines.lines().map(|l| l.chars().collect()).collect();

        Grid(data)
    }

    fn part_1(parsed_input: &mut Self::ParsedInput) -> String {
        format!("{}", parsed_input.get_part_numbers().iter().sum::<usize>())
    }

    fn part_2(parsed_input: &mut Self::ParsedInput) -> String {
        format!("{}", parsed_input.get_ratios().iter().sum::<usize>())
    }
}
