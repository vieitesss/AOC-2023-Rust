use std::{collections::HashMap, fmt::Display};

use crate::Solution;

pub struct Day14;

#[derive(Debug, Clone, Hash)]
struct Platform(Vec<Vec<char>>);

#[derive(Debug, Clone)]
pub struct Matrix {
    matrix: Platform,
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (_, line) in self.matrix.0.iter().enumerate() {
            for (col, ch) in line.iter().enumerate() {
                if col == line.len() - 1 {
                    writeln!(f, "{ch}")?;
                    break;
                }
                write!(f, "{ch}")?;
            }
        }

        Ok(())
    }
}

#[derive(Hash, Eq, Debug, PartialEq, Clone, Copy)]
enum DIRECTION {
    NORTH,
    SOUTH,
    WEST,
    EAST,
}

const SPIN_CYCLE: [DIRECTION; 4] = [
    DIRECTION::NORTH,
    DIRECTION::WEST,
    DIRECTION::SOUTH,
    DIRECTION::EAST,
];

impl Matrix {
    fn tilt(&mut self, dir: DIRECTION) {
        let mut next_pos: usize;
        let len = self.matrix.0.len() - 1;
        if dir == DIRECTION::NORTH || dir == DIRECTION::WEST {
            next_pos = 0;
        } else {
            next_pos = len;
        }

        for (row, line) in self.matrix.0.clone().iter().enumerate() {
            for (col, _) in line.iter().enumerate() {
                let ch = if dir == DIRECTION::NORTH {
                    self.matrix.0[col][row]
                } else if dir == DIRECTION::WEST {
                    self.matrix.0[row][col]
                } else if dir == DIRECTION::SOUTH {
                    self.matrix.0[len - col][row]
                } else {
                    self.matrix.0[row][len - col]
                };

                match ch {
                    '.' => {}
                    '#' => {
                        if dir == DIRECTION::NORTH || dir == DIRECTION::WEST {
                            next_pos = col + 1;
                        } else {
                            next_pos = len - col - 1;
                        }
                    }
                    'O' => {
                        if dir == DIRECTION::NORTH {
                            self.matrix.0[col][row] = '.';
                            self.matrix.0[next_pos][row] = 'O';
                            next_pos += 1;
                        } else if dir == DIRECTION::WEST {
                            self.matrix.0[row][col] = '.';
                            self.matrix.0[row][next_pos] = 'O';
                            next_pos += 1;
                        } else if dir == DIRECTION::SOUTH {
                            self.matrix.0[len - col][row] = '.';
                            self.matrix.0[next_pos][row] = 'O';
                            next_pos -= 1;
                        } else {
                            self.matrix.0[row][len - col] = '.';
                            self.matrix.0[row][next_pos] = 'O';
                            next_pos -= 1;
                        }
                    }
                    _ => panic!("not a valid char {}", ch),
                }
            }

            if dir == DIRECTION::NORTH || dir == DIRECTION::WEST {
                next_pos = 0;
            } else {
                next_pos = len;
            }
        }
    }

    fn load_on_north(&self) -> usize {
        let mut current_val = self.matrix.0.len();

        self.matrix.0.iter().fold(0, |cur, line| {
            let total =
                cur + line.iter().fold(0, |cur, ch| cur + (*ch == 'O') as usize) * current_val;
            current_val -= 1;
            total
        })
    }
}

impl Solution for Day14 {
    type ParsedInput = Matrix;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let mut matrix = Vec::new();

        for line in input_lines.lines() {
            let mut row: Vec<char> = Vec::new();
            for ch in line.chars() {
                row.push(ch);
            }
            matrix.push(row);
        }

        Matrix {
            matrix: Platform(matrix),
        }
    }

    fn part_1(parsed_input: &Self::ParsedInput) -> String {
        let mut cloned = parsed_input.clone();
        cloned.tilt(DIRECTION::NORTH);
        cloned.load_on_north().to_string()
    }

    fn part_2(mut parsed_input: Self::ParsedInput) -> String {
        let mut times = 1_000_000_000;
        let mut is_counting = false;
        let mut count = 1;
        let mut memo: HashMap<Vec<Vec<char>>, usize> = HashMap::new();

        while times > 0 {
            SPIN_CYCLE.iter().for_each(|dir| {
                parsed_input.tilt(*dir);

                let key = parsed_input.matrix.0.clone();

                if is_counting {
                    if *memo.get(&key).unwrap() == 1 {
                        is_counting = false;
                        times = times % count;
                    }
                    count += 1;
                    return;
                }

                if memo.get(&key).is_none() {
                    memo.insert(key, 0);
                } else {
                    memo.insert(key, 1);
                    is_counting = true;
                }
            });

            times -= 1;
        }

        parsed_input.load_on_north().to_string()
    }
}
