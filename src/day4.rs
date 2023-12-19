use crate::Solution;

pub struct Day4;

#[derive(Debug)]
pub struct Card {
    winning: Vec<String>,
    playing: Vec<String>,
}

impl Card {
    fn get_matches(&self) -> usize {
        self.winning.iter().fold(0, |cur, n| {
            if self.playing.contains(n) {
                return cur + 1;
            }

            cur
        })
    }

    fn get_points(&self) -> usize {
        let n = self.get_matches();

        if n > 2 {
            return vec![2; (n - 1) as usize].iter().fold(1, |a, b| a * b);
        }

        n
    }
}

#[derive(Debug)]
pub struct Table(Vec<Card>);

impl Table {
    fn get_all_points(&self) -> String {
        self.0
            .iter()
            .map(|c| c.get_points())
            .sum::<usize>()
            .to_string()
    }

    fn get_all_instances(&self) -> String {
        let mut instances = vec![1; self.0.len()];

        for (i, card) in self.0.iter().enumerate() {
            let matches = card.get_matches();
            for x in (i + 1)..=(i + matches) {
                instances[x] += 1 * instances[i];
            }
        }

        instances.iter().sum::<usize>().to_string()
    }
}

impl Solution for Day4 {
    type ParsedInput = Table;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let mut cards: Vec<Card> = vec![];

        for line in input_lines.lines() {
            let numbers: Vec<&str> = line.split([':']).nth(1).unwrap().split('|').collect();
            let c = Card {
                winning: numbers[0]
                    .trim()
                    .split_whitespace()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>(),
                playing: numbers[1]
                    .trim()
                    .split_whitespace()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>(),
            };
            cards.push(c);
        }

        Table(cards)
    }

    fn part_1(parsed_input: &Self::ParsedInput) -> String {
        parsed_input.get_all_points()
    }

    fn part_2(parsed_input: Self::ParsedInput) -> String {
        parsed_input.get_all_instances()
    }
}

