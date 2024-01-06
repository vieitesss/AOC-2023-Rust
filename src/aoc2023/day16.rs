use std::collections::HashMap;

use crate::Solution;

#[derive(PartialEq, Copy, Clone, Debug)]
enum DIR {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

impl DIR {
    fn values(&self) -> (isize, isize) {
        match *self {
            DIR::UP => (-1, 0),
            DIR::DOWN => (1, 0),
            DIR::RIGHT => (0, 1),
            DIR::LEFT => (0, -1),
        }
    }
}

#[derive(Copy, Clone)]
enum SYMBOL {
    POINT,
    SLASH,
    SLASHB,
    DASH,
    BAR,
}

pub struct Matrix(Vec<Vec<SYMBOL>>);

type Memo = HashMap<(isize, isize), DIR>;

impl Matrix {
    fn next_dir(&self, dir: DIR, x: isize, y: isize) -> Vec<DIR> {
        match (dir, self.0[x as usize][y as usize]) {
            (DIR::UP, SYMBOL::POINT) => vec![DIR::UP],
            (DIR::UP, SYMBOL::SLASH) => vec![DIR::RIGHT],
            (DIR::UP, SYMBOL::SLASHB) => vec![DIR::LEFT],
            (DIR::UP, SYMBOL::DASH) => vec![DIR::LEFT, DIR::RIGHT],
            (DIR::UP, SYMBOL::BAR) => vec![DIR::UP],
            (DIR::DOWN, SYMBOL::POINT) => vec![DIR::DOWN],
            (DIR::DOWN, SYMBOL::SLASH) => vec![DIR::LEFT],
            (DIR::DOWN, SYMBOL::SLASHB) => vec![DIR::RIGHT],
            (DIR::DOWN, SYMBOL::DASH) => vec![DIR::LEFT, DIR::RIGHT],
            (DIR::DOWN, SYMBOL::BAR) => vec![DIR::DOWN],
            (DIR::RIGHT, SYMBOL::POINT) => vec![DIR::RIGHT],
            (DIR::RIGHT, SYMBOL::SLASH) => vec![DIR::UP],
            (DIR::RIGHT, SYMBOL::SLASHB) => vec![DIR::DOWN],
            (DIR::RIGHT, SYMBOL::DASH) => vec![DIR::RIGHT],
            (DIR::RIGHT, SYMBOL::BAR) => vec![DIR::UP, DIR::DOWN],
            (DIR::LEFT, SYMBOL::POINT) => vec![DIR::LEFT],
            (DIR::LEFT, SYMBOL::SLASH) => vec![DIR::DOWN],
            (DIR::LEFT, SYMBOL::SLASHB) => vec![DIR::UP],
            (DIR::LEFT, SYMBOL::DASH) => vec![DIR::LEFT],
            (DIR::LEFT, SYMBOL::BAR) => vec![DIR::UP, DIR::DOWN],
        }
    }

    fn is_out_of_bounds(&self, point: &(isize, isize)) -> bool {
        return point.0 < 0
            || point.0 > (self.0.len() - 1) as isize
            || point.1 < 0
            || point.1 > (self.0[0].len() - 1) as isize;
    }

    fn energized_tiles_with_config(&self, dir: DIR, point: &(usize, usize)) -> usize {
        self.energized_tiles(
            dir,
            &(point.0 as isize, point.1 as isize),
            &mut HashMap::new(),
        )
    }

    fn energized_tiles(&self, dir: DIR, point: &(isize, isize), memo: &mut Memo) -> usize {
        if self.is_out_of_bounds(point) {
            return 0;
        }

        let cell_type = self.0[point.0 as usize][point.1 as usize];
        let next_dir = self.next_dir(dir, point.0, point.1);

        if let Some(memoized) = memo.get(&point) {
            if *memoized == dir {
                return 0;
            }

            memo.insert(*point, dir);
            match cell_type {
                SYMBOL::POINT | SYMBOL::SLASH | SYMBOL::SLASHB => {
                    let values = next_dir[0].values();
                    return self.energized_tiles(
                        next_dir[0],
                        &(point.0 + values.0, point.1 + values.1),
                        memo,
                    );
                }
                SYMBOL::DASH | SYMBOL::BAR => return 0,
            };
        }

        memo.insert(*point, dir);

        next_dir.iter().for_each(|d| {
            let values = d.values();
            self.energized_tiles(*d, &(point.0 + values.0, point.1 + values.1), memo);
        });

        return memo.len();
    }
}

pub struct Day16;

impl Solution for Day16 {
    type ParsedInput = Matrix;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        Matrix(
            input_lines
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => SYMBOL::POINT,
                            '/' => SYMBOL::SLASH,
                            '\\' => SYMBOL::SLASHB,
                            '|' => SYMBOL::BAR,
                            '-' => SYMBOL::DASH,
                            _ => panic!("not a symbol {c}"),
                        })
                        .collect()
                })
                .collect(),
        )
    }

    fn part_1(parsed_input: &Self::ParsedInput) -> String {
        parsed_input
            .energized_tiles(DIR::RIGHT, &(0, 0), &mut HashMap::new())
            .to_string()
    }

    fn part_2(parsed_input: Self::ParsedInput) -> String {
        parsed_input
            .0
            .iter()
            .enumerate()
            .map(|(x, line)| {
                line.iter()
                    .enumerate()
                    .map(|(y, _)| {
                        if x == 0 && y == 0 {
                            let res1 =
                                parsed_input.energized_tiles_with_config(DIR::RIGHT, &(x, y));
                            let res2 = parsed_input.energized_tiles_with_config(DIR::DOWN, &(x, y));
                            return if res1 > res2 { res1 } else { res2 };
                        } else if x == 0 && y == line.len() - 1 {
                            let res1 = parsed_input.energized_tiles_with_config(DIR::LEFT, &(x, y));
                            let res2 = parsed_input.energized_tiles_with_config(DIR::DOWN, &(x, y));
                            return if res1 > res2 { res1 } else { res2 };
                        } else if x == 0 {
                            return parsed_input.energized_tiles_with_config(DIR::DOWN, &(x, y));
                        } else if y == 0 && x == parsed_input.0.len() - 1 {
                            let res1 =
                                parsed_input.energized_tiles_with_config(DIR::RIGHT, &(x, y));
                            let res2 = parsed_input.energized_tiles_with_config(DIR::UP, &(x, y));
                            return if res1 > res2 { res1 } else { res2 };
                        } else if y == 0 {
                            return parsed_input.energized_tiles_with_config(DIR::RIGHT, &(x, y));
                        } else if y == line.len() - 1 && x == parsed_input.0.len() - 1 {
                            let res1 = parsed_input.energized_tiles_with_config(DIR::LEFT, &(x, y));
                            let res2 = parsed_input.energized_tiles_with_config(DIR::UP, &(x, y));
                            return if res1 > res2 { res1 } else { res2 };
                        } else if y == line.len() - 1 {
                            return parsed_input.energized_tiles_with_config(DIR::LEFT, &(x, y));
                        } else if x == parsed_input.0.len() - 1 {
                            return parsed_input.energized_tiles_with_config(DIR::UP, &(x, y));
                        }

                        0
                    })
                    .collect::<Vec<usize>>()
                    .iter()
                    .fold(0, |acc, i| if *i > acc { *i } else { acc })
            })
            .collect::<Vec<_>>()
            .iter()
            .fold(0, |acc, i| if *i > acc { *i } else { acc })
            .to_string()
    }
}

