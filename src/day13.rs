#![allow(dead_code)]
#![allow(unused_variables)]

use crate::Solution;

pub struct Day13;

#[derive(Debug)]
struct Matrix {
    rows: Vec<String>,
    columns: Vec<String>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum REFLECTION {
    ROWS,
    COLUMNS,
}

impl Matrix {
    fn is_reflection(&self, ref_type: REFLECTION, mut start: usize, mut end: usize) -> bool {
        let vector = if ref_type == REFLECTION::ROWS {
            self.rows.clone()
        } else {
            self.columns.clone()
        };

        while start < end {
            // println!("{} - {}", vector[start], vector[end]);
            if vector[start] != vector[end] {
                return false;
            }
            start += 1;
            end -= 1;
        }

        if start == end {
            return false;
        }

        true
    }

    fn get_line_of_reflection(&self, ref_type: REFLECTION) -> Option<usize> {
        let vector = if ref_type == REFLECTION::ROWS {
            self.rows.clone()
        } else {
            self.columns.clone()
        };

        let first = vector.first()?;
        let last = vector.last()?;

        for index in 0..vector.iter().len() {
            if *first == vector[index] && index != 0 && self.is_reflection(ref_type, 0, index) {
                return Some((index + 1) / 2);
            }
            if *last == vector[index]
                && index != vector.len() - 1
                && self.is_reflection(ref_type, index, vector.len() - 1)
            {
                return Some((index + vector.len()) / 2);
            }
        }

        None
    }
}

#[derive(Debug)]
pub struct Data(Vec<Matrix>);

impl Data {
    fn summarize(&self) -> usize {
        self.0.iter().fold(0, |cur, matrix| {
            if let Some(index) = matrix.get_line_of_reflection(REFLECTION::ROWS) {
                return cur + index * 100;
            }

            if let Some(index) = matrix.get_line_of_reflection(REFLECTION::COLUMNS) {
                return cur + index;
            }

            cur
        })
    }
}

impl Solution for Day13 {
    type ParsedInput = Data;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let mut rows: Vec<String> = vec![];
        let mut columns: Vec<String> = vec![];
        let mut data: Vec<Matrix> = vec![];
        for line in input_lines.lines() {
            if line.is_empty() {
                data.push(Matrix {
                    rows: rows.clone(),
                    columns: columns.clone(),
                });
                rows.clear();
                columns.clear();
                continue;
            }
            rows.push(line.to_string());
            for (col, ch) in line.chars().enumerate() {
                if let Some(column) = columns.get(col) {
                    columns[col] = format!("{}{}", column, ch);
                } else {
                    columns.push(ch.to_string());
                }
            }
        }

        data.push(Matrix { rows, columns });

        Data(data)
    }

    fn part_1(parsed_input: &Self::ParsedInput) -> String {
        parsed_input.summarize().to_string()
    }

    fn part_2(parsed_input: Self::ParsedInput) -> String {
        "".to_string()
    }
}
