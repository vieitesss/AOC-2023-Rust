use crate::Solution;

pub struct Day4;

#[derive(Debug)]
pub struct Card {
    id: usize,
    winning: Vec<String>,
    playing: Vec<String>,
}

impl Card {
    fn get_points(&self) -> usize {
        let mut n: usize = 0;
        for w in self.winning.iter() {
            if self.playing.contains(w) {
                n += 1;
            }
        }
        match n {
            0..=2 => n,
            _ => vec![2; (n - 1) as usize].iter().fold(1, |a, b| a * b),
        }
    }
}

impl Solution for Day4 {
    type ParsedInput = Vec<Card>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let mut cards: Vec<Card> = vec![];

        for line in input_lines.lines() {
            let id_numbers = line.split([':', '|']).collect::<Vec<&str>>();
            let c = Card {
                id: id_numbers[0]
                    .split_whitespace()
                    .nth(1)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
                winning: id_numbers[1]
                    .trim()
                    .split_whitespace()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>(),
                playing: id_numbers[2]
                    .trim()
                    .split_whitespace()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>(),
            };
            cards.push(c);
        }

        cards
    }

    fn part_1(parsed_input: &mut Self::ParsedInput) -> String {
        format!(
            "{}",
            parsed_input.iter().map(|c| c.get_points()).sum::<usize>()
        )
    }

    fn part_2(parsed_input: &mut Self::ParsedInput) -> String {
        "".to_string()
    }
}

