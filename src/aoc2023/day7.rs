use crate::Solution;

use std::collections::HashMap;

pub struct Day7;

const CARDS: [char; 5] = ['T', 'J', 'Q', 'K', 'A'];

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

const ORDERED_TYPES: [Type; 7] = [
    Type::HighCard,
    Type::OnePair,
    Type::TwoPair,
    Type::ThreeOfAKind,
    Type::FullHouse,
    Type::FourOfAKind,
    Type::FiveOfAKind,
];

#[derive(Debug, PartialEq)]
enum Compare {
    HIGHER,
    EQUAL,
    LOWER,
}

#[derive(Debug, Clone)]
struct Line {
    hand: String,
    hand_type: Type,
    bid: usize,
}

impl Line {
    fn new(hand: String, hand_type: Type, bid: usize) -> Self {
        Self {
            hand,
            hand_type,
            bid,
        }
    }

    fn compare_hand(&self, other_hand: String, with_jokers: bool) -> Compare {
        let other_hand_chars: Vec<char> = other_hand.chars().collect();
        let self_hand_chars: Vec<char> = self.hand.chars().collect();
        for i in 0..other_hand.len() {
            let ohc = other_hand_chars[i];
            let shc = self_hand_chars[i];
            if let Some(compare) = self.compare_chars(shc, ohc, with_jokers) {
                return compare;
            }
        }

        Compare::EQUAL
    }

    fn compare_chars(
        &self,
        self_char: char,
        other_char: char,
        with_jokers: bool,
    ) -> Option<Compare> {
        if with_jokers {
            if self_char == 'J' && other_char != 'J' {
                return Some(Compare::LOWER);
            } else if self_char != 'J' && other_char == 'J' {
                return Some(Compare::HIGHER);
            }
        }

        if other_char.is_digit(10) && self_char.is_digit(10) {
            if other_char.to_digit(10).unwrap() > self_char.to_digit(10).unwrap() {
                return Some(Compare::LOWER);
            }
            if other_char.to_digit(10).unwrap() < self_char.to_digit(10).unwrap() {
                return Some(Compare::HIGHER);
            }
        } else if other_char.is_digit(10) && !self_char.is_digit(10) {
            return Some(Compare::HIGHER);
        } else if !other_char.is_digit(10) && self_char.is_digit(10) {
            return Some(Compare::LOWER);
        } else {
            let self_index = CARDS.iter().position(|c| *c == self_char).unwrap();
            let other_index = CARDS.iter().position(|c| *c == other_char).unwrap();

            if self_index > other_index {
                return Some(Compare::HIGHER);
            }
            if self_index < other_index {
                return Some(Compare::LOWER);
            }
        }

        None
    }

    fn get_hand_type_with_jokers(&mut self) -> Type {
        let mut map: HashMap<char, usize> = HashMap::new();
        let jokers = self
            .hand
            .chars()
            .collect::<Vec<char>>()
            .iter()
            .fold(0, |cur, c| {
                if *c == 'J' {
                    return cur + 1;
                }

                if map.contains_key(c) {
                    map.insert(*c, map.get(c).unwrap() + 1);
                } else {
                    map.insert(*c, 1);
                }

                cur
            });

        let len = map.len();
        if jokers == 5
            || jokers == 4
            || (jokers == 3 && len == 1)
            || (jokers == 2 && len == 1)
            || (jokers == 1 && len == 1)
        {
            return Type::FiveOfAKind;
        } else if jokers == 3
            || (jokers == 2 && len == 2)
            || (jokers == 1 && len == 2 && map.values().max() == Some(&3))
        {
            return Type::FourOfAKind;
        } else if jokers == 2 || (jokers == 1 && len == 3) {
            return Type::ThreeOfAKind;
        } else if jokers == 1 && len == 2 {
            return Type::FullHouse;
        } else if jokers == 1 {
            return Type::OnePair;
        }

        return self.hand_type.clone();
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    types: HashMap<Type, Vec<Line>>,
    lines: Vec<Line>,
}

impl Game {
    fn new(lines: Vec<Line>) -> Self {
        Self {
            types: HashMap::new(),
            lines,
        }
    }

    fn get_total_winnings(&mut self, with_jokers: bool) -> String {
        self.sort_hands(with_jokers);

        // println!("{:#?}", self.types);

        let mut factor = 1;
        let mut total = 0;
        for hand_type in ORDERED_TYPES.iter() {
            if let Some(vector) = self.types.get(hand_type) {
                for l in vector.iter() {
                    total += l.bid * factor;
                    factor += 1;
                }
            }
        }

        total.to_string()
    }

    fn sort_hands(&mut self, with_jokers: bool) {
        for line in self.lines.clone().iter() {
            let hand_type = &line.hand_type;
            if self.types.contains_key(&hand_type) {
                self.insert_sorted(line.clone(), hand_type.clone(), with_jokers);
            } else {
                self.types.insert(hand_type.clone(), vec![line.clone()]);
            }
        }
    }

    fn insert_sorted(&mut self, hand: Line, hand_type: Type, with_jokers: bool) {
        let mut vector: Vec<Line> = self.types.get(&hand_type).unwrap().to_vec();
        let mut left = 0;
        let mut right = vector.len() - 1;
        let mut middle: usize;

        loop {
            middle = (right + left) / 2;
            match hand.compare_hand(vector[middle].hand.clone(), with_jokers) {
                Compare::EQUAL => {
                    vector.insert(middle, hand);
                    break;
                }
                Compare::LOWER => {
                    if middle == left {
                        vector.insert(middle, hand);
                        break;
                    }

                    right = middle - 1;
                }
                Compare::HIGHER => {
                    if middle == right {
                        vector.insert(middle + 1, hand);
                        break;
                    }

                    left = middle + 1;
                }
            }
        }

        self.types.insert(hand_type, vector);
    }
}

fn get_hand_type(hand: String) -> Type {
    let mut map: HashMap<char, usize> = HashMap::new();
    for c in hand.chars() {
        if map.contains_key(&c) {
            map.insert(c, map.get(&c).unwrap() + 1);
        } else {
            map.insert(c, 1);
        }
    }

    match map.len() {
        5 => Type::HighCard,
        4 => Type::OnePair,
        3 => {
            if let Some(_) = map.keys().find(|k| *map.get(k).unwrap() == 2) {
                Type::TwoPair
            } else {
                Type::ThreeOfAKind
            }
        }
        2 => {
            if let Some(_) = map
                .keys()
                .find(|k| *map.get(k).unwrap() == 2 || *map.get(k).unwrap() == 3)
            {
                Type::FullHouse
            } else {
                Type::FourOfAKind
            }
        }
        1 => Type::FiveOfAKind,
        _ => panic!("imposible type"),
    }
}

impl Solution for Day7 {
    type ParsedInput = Game;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let lines = input_lines
            .lines()
            .map(|l| {
                let mut items = l.split_whitespace();
                let hand = items.next().unwrap().to_string();
                Line::new(
                    hand.clone(),
                    get_hand_type(hand.clone()),
                    items.next().unwrap().parse().unwrap(),
                )
            })
            .collect::<Vec<Line>>();

        Game::new(lines)
    }

    fn part_1(parsed_input: &Self::ParsedInput) -> String {
        let mut game = parsed_input.clone();
        game.get_total_winnings(false)
    }

    fn part_2(parsed_input: Self::ParsedInput) -> String {
        let mut input = parsed_input;
        for line in input.lines.iter_mut() {
            if line.hand.contains('J') {
                line.hand_type = line.get_hand_type_with_jokers();
            }
        }

        input.get_total_winnings(true)
    }
}
