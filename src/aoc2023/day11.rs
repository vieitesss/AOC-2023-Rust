use crate::Solution;

pub struct Day11;

#[derive(Debug)]
pub struct Data {
    galaxies: Vec<(i64, i64)>,
    empty_rows: Vec<i64>,
    empty_columns: Vec<i64>,
}

impl Data {
    fn distance_to_following(&self, galaxy: usize, target: usize, total: i64, inc: i64) -> i64 {
        if target == self.galaxies.len() {
            return total;
        }

        let gp = self.galaxies[galaxy];
        let tp = self.galaxies[target];

        let mut distance = (tp.0 - gp.0) + (tp.1 - gp.1).abs();

        for r in self.empty_rows.iter() {
            if *r > gp.0 && *r < tp.0 {
                distance += inc;
            }
        }

        for c in self.empty_columns.iter() {
            if *c > gp.1 && *c < tp.1 || *c < gp.1 && *c > tp.1 {
                distance += inc;
            }
        }

        self.distance_to_following(galaxy, target + 1, total + distance, inc)
    }

    fn distance_btwn_galaxies(&self, inc: i64) -> i64 {
        self.galaxies.iter().enumerate().fold(0, |cur, (i, _)| {
            cur + self.distance_to_following(i, i + 1, 0, inc)
        })
    }
}

impl Solution for Day11 {
    type ParsedInput = Data;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let mut galaxies = vec![];
        let mut empty_rows = vec![];
        let mut empty_columns: Vec<i64> =
            (0..input_lines.lines().count()).map(|i| i as i64).collect();
        for (row, line) in input_lines.lines().enumerate() {
            if line.chars().enumerate().fold(0, |cur, (col, c)| {
                if c == '#' {
                    galaxies.push((row as i64, col as i64));
                    if let Some(index) = empty_columns.iter().position(|c| *c == col as i64) {
                        empty_columns.remove(index);
                    }
                    return cur;
                }
                cur + 1
            }) == line.len() as i64
            {
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
        parsed_input.distance_btwn_galaxies(1).to_string()
    }

    fn part_2(parsed_input: Self::ParsedInput) -> String {
        parsed_input.distance_btwn_galaxies(999_999).to_string()
    }
}
