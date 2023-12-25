#![allow(dead_code)]

use crate::Solution;

pub struct Day14;

#[derive(Debug)]
pub struct Matrix {
    rows: Vec<String>,
    columns: Vec<String>,
}

impl Matrix {
    fn load_on_north(&self) -> usize {
        let len = self.columns.len();
        let mut current_val = len;
        let mut total = 0;

        for column in self.columns.iter() {
            for (row, ch) in column.chars().enumerate() {
                match ch {
                    '.' => {}
                    '#' => current_val = len - row - 1,
                    'O' => {
                        total += current_val;
                        current_val -= 1;
                    }
                    _ => panic!("not a valid char {}", ch),
                }
            }

            current_val = len;
        }

        total
    }
}

impl Solution for Day14 {
    type ParsedInput = Matrix;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let mut rows: Vec<String> = vec![];
        let mut columns: Vec<String> = vec![];
        for line in input_lines.lines() {
            rows.push(line.to_string());
            for (col, ch) in line.chars().enumerate() {
                if let Some(column) = columns.get(col) {
                    columns[col] = format!("{}{}", column, ch);
                } else {
                    columns.push(ch.to_string());
                }
            }
        }

        Matrix { rows, columns }
    }

    fn part_1(parsed_input: &Self::ParsedInput) -> String {
        parsed_input.load_on_north().to_string()
    }

    fn part_2(_parsed_input: Self::ParsedInput) -> String {
        "".to_string()
    }
}

