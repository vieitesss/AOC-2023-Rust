use crate::Solution;

pub struct Day11;

#[derive(Debug)]
pub struct Data {
    galaxies: Vec<(i32, i32)>,
    empty_rows: Vec<i32>,
    empty_columns: Vec<i32>,
}

impl Data {
    fn distance_to_following(&self, galaxy: usize, target: usize, total: i32) -> i32 {
        if target == self.galaxies.len() {
            return total;
        }

        let galaxy_point = self.galaxies[galaxy];
        let target_point = self.galaxies[target];

        let mut distance =
            (target_point.0 - galaxy_point.0) + (target_point.1 - galaxy_point.1).abs();

        for r in self.empty_rows.iter() {
            if *r > galaxy_point.0 && *r < target_point.0 {
                distance += 1;
            }
        }

        for c in self.empty_columns.iter() {
            if *c > galaxy_point.1 && *c < target_point.1
                || *c < galaxy_point.1 && *c > target_point.1
            {
                distance += 1;
            }
        }

        self.distance_to_following(galaxy, target + 1, total + distance)
    }
}

impl Solution for Day11 {
    type ParsedInput = Data;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let mut galaxies = vec![];
        let mut empty_rows = vec![];
        let mut empty_columns: Vec<i32> =
            (0..input_lines.lines().count()).map(|i| i as i32).collect();
        let mut space: i32;
        for (row, line) in input_lines.lines().enumerate() {
            space = 0;
            for (col, c) in line.chars().enumerate() {
                if c == '#' {
                    galaxies.push((row as i32, col as i32));
                    if let Some(index) = empty_columns.iter().position(|c| *c == col as i32) {
                        empty_columns.remove(index);
                    }
                } else {
                    space += 1;
                }
            }
            if space == line.len() as i32 {
                empty_rows.push(row as i32);
            }
        }

        Data {
            galaxies,
            empty_rows,
            empty_columns,
        }
    }

    fn part_1(parsed_input: &Self::ParsedInput) -> String {
        parsed_input
            .galaxies
            .iter()
            .enumerate()
            .fold(0, |cur, (i, _)| {
                cur + parsed_input.distance_to_following(i, i + 1, 0)
            })
            .to_string()
    }

    fn part_2(_parsed_input: Self::ParsedInput) -> String {
        "".to_string()
    }
}

