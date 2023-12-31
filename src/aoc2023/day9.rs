use crate::Solution;

#[derive(Debug)]
pub struct History(Vec<i32>);

impl History {
    fn extrapolate(&self, backwards: bool) -> Option<i32> {
        let values = self.get_next_values(&self.0);

        if let Some(v) = self.rec(&values, backwards) {
            if backwards {
                return Some(self.0.first().unwrap() - v);
            }
            return Some(v + self.0.last().unwrap());
        }

        None
    }

    fn rec(&self, values: &[i32], backwards: bool) -> Option<i32> {
        if values.len() == 1 && values[0] != 0 {
            return None;
        }

        if values.iter().all(|v| *v == values[0]) && values.len() > 1 {
            return Some(values[0]);
        }

        if let Some(v) = self.rec(&self.get_next_values(&values), backwards) {
            if backwards {
                return Some(values.first().unwrap() - v);
            }
            return Some(values.last().unwrap() + v);
        }

        None
    }

    fn get_next_values(&self, current: &[i32]) -> Vec<i32> {
        let mut values: Vec<i32> = vec![];
        for i in 0..current.len() - 1 {
            values.push(-1 * (current[i] - current[i + 1]));
        }

        values
    }
}

pub struct Day9;

impl Solution for Day9 {
    type ParsedInput = Vec<History>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines
            .lines()
            .map(|l| {
                History(
                    l.split_whitespace()
                        .map(|n| n.parse::<i32>().unwrap())
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<History>>()
    }

    fn part_1(parsed_input: &Self::ParsedInput) -> String {
        parsed_input
            .iter()
            .fold(0, |cur, h| {
                if let Some(r) = h.extrapolate(false) {
                    cur + r
                } else {
                    cur + h.0.last().unwrap()
                }
            })
            .to_string()
    }

    fn part_2(parsed_input: Self::ParsedInput) -> String {
        parsed_input
            .iter()
            .fold(0, |cur, h| {
                if let Some(r) = h.extrapolate(true) {
                    cur + r
                } else {
                    cur + h.0.first().unwrap()
                }
            })
            .to_string()
    }
}
