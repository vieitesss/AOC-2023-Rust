use crate::Solution;

pub struct Day11;

#[derive(Debug)]
pub struct Data {
    galaxies: Vec<(i64, i64)>,
    empty_rows: Vec<i64>,
    empty_columns: Vec<i64>,
}

impl Data {
    fn distance_to_following(
        &self,
        galaxy: usize,
        target: usize,
        total: i64,
        increase: i64,
    ) -> i64 {
        if target == self.galaxies.len() {
            return total;
        }

        let galaxy_point = self.galaxies[galaxy];
        let target_point = self.galaxies[target];

        let mut distance =
            (target_point.0 - galaxy_point.0) + (target_point.1 - galaxy_point.1).abs();

        for r in self.empty_rows.iter() {
            if *r > galaxy_point.0 && *r < target_point.0 {
                distance += increase;
            }
        }

        for c in self.empty_columns.iter() {
            if *c > galaxy_point.1 && *c < target_point.1
                || *c < galaxy_point.1 && *c > target_point.1
            {
                distance += increase;
            }
        }

        self.distance_to_following(galaxy, target + 1, total + distance, increase)
    }
}

impl Solution for Day11 {
    type ParsedInput = Data;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let mut galaxies = vec![];
        let mut empty_rows = vec![];
        let mut empty_columns: Vec<i64> =
            (0..input_lines.lines().count()).map(|i| i as i64).collect();
        let mut space: i64;
        for (row, line) in input_lines.lines().enumerate() {
            space = 0;
            for (col, c) in line.chars().enumerate() {
                if c == '#' {
                    galaxies.push((row as i64, col as i64));
                    if let Some(index) = empty_columns.iter().position(|c| *c == col as i64) {
                        empty_columns.remove(index);
                    }
                } else {
                    space += 1;
                }
            }
            if space == line.len() as i64 {
                empty_rows.push(row as i64);
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
                cur + parsed_input.distance_to_following(i, i + 1, 0, 1)
            })
            .to_string()
    }

    fn part_2(parsed_input: Self::ParsedInput) -> String {
        parsed_input
            .galaxies
            .iter()
            .enumerate()
            .fold(0, |cur, (i, _)| {
                cur + parsed_input.distance_to_following(i, i + 1, 0, 999_999)
            })
            .to_string()
    }
}
