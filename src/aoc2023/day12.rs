use std::collections::HashMap;

use crate::Solution;

pub struct Day12;

#[derive(Debug)]
struct Record {
    condition: String,
    groups: Vec<usize>,
}

type Memo = HashMap<(String, Vec<usize>), usize>;

impl Record {
    fn find_arrangements(&self, cond: &str, groups: &[usize], memo: &mut Memo) -> usize {
        if let Some(memoized) = memo.get(&(cond.to_string(), groups.to_vec())) {
            return *memoized;
        }

        if groups.is_empty() && (cond.is_empty() || !cond.contains('#')) {
            return 1;
        }

        let space_needed = groups.iter().sum::<usize>() + groups.len() - 1;

        if cond.len() < space_needed {
            return 0;
        }

        let current = cond.chars().next().unwrap();
        let next_slice = cond.split_at(1).1;
        let result = match current {
            '.' => self.find_arrangements(next_slice, groups, memo),
            '?' => {
                self.find_arrangements(next_slice, groups, memo)
                    + self.found_hash(cond, groups, memo)
            }
            '#' => self.found_hash(cond, groups, memo),
            _ => panic!("Not a symbol: {current}"),
        };

        memo.insert((cond.to_string(), groups.to_vec()), result);
        result
    }

    fn found_hash(&self, cond: &str, groups: &[usize], memo: &mut Memo) -> usize {
        if let Some(ch) = cond.chars().nth(groups[0]) {
            if ch == '#' {
                return 0;
            }
        }

        if cond.split_at(groups[0]).0.contains('.') {
            return 0;
        }

        if cond.len() == groups[0] {
            return 1;
        }

        self.find_arrangements(cond.split_at(groups[0] + 1).1, &groups[1..], memo)
    }
}

#[derive(Debug)]
pub struct Records(Vec<Record>);

impl Records {
    fn sum_of_arrangements(&self) -> usize {
        self.0.iter().fold(0, |cur, r| {
            cur + r.find_arrangements(&r.condition, &r.groups, &mut Memo::new())
        })
    }

    fn extend(&self, record: &Record) -> Record {
        let mut new_cond = record.condition.clone();
        let mut new_grps = record.groups.clone();
        for _ in 0..4 {
            new_cond = format!("{}?{}", new_cond, record.condition);
            record.groups.iter().for_each(|g| new_grps.push(*g));
        }
        Record {
            condition: new_cond,
            groups: new_grps,
        }
    }

    fn sum_by_five(&self) -> usize {
        let mut result = 0;
        for r in self.0.iter() {
            let new_r = self.extend(&r);
            result += new_r.find_arrangements(&new_r.condition, &new_r.groups, &mut Memo::new());
        }

        result
    }
}

impl Solution for Day12 {
    type ParsedInput = Records;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        Records(
            input_lines
                .lines()
                .map(|line| {
                    let cond_grps = line.split_whitespace().collect::<Vec<&str>>();

                    Record {
                        condition: cond_grps[0].to_string(),
                        groups: cond_grps[1]
                            .split(',')
                            .map(|n| n.parse().unwrap())
                            .collect(),
                    }
                })
                .collect(),
        )
    }

    fn part_1(parsed_input: &Self::ParsedInput) -> String {
        parsed_input.sum_of_arrangements().to_string()
    }

    fn part_2(parsed_input: Self::ParsedInput) -> String {
        parsed_input.sum_by_five().to_string()
    }
}
