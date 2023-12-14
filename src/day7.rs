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

#[derive(Debug)]
enum CmpHand {
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

    fn compare(&self, other_hand: String) -> CmpHand {
        let other_hand_chars: Vec<char> = other_hand.chars().collect();
        let self_hand_chars: Vec<char> = self.hand.chars().collect();
        for i in 0..other_hand.len() {
            let ohc = other_hand_chars[i];
            let shc = self_hand_chars[i];
            if ohc.is_digit(10) && shc.is_digit(10) {
                if ohc.to_digit(10).unwrap() > shc.to_digit(10).unwrap() {
                    return CmpHand::LOWER;
                } else if ohc.to_digit(10).unwrap() < shc.to_digit(10).unwrap() {
                    return CmpHand::HIGHER;
                }
            } else if ohc.is_digit(10) && !shc.is_digit(10) {
                return CmpHand::HIGHER;
            } else if !ohc.is_digit(10) && shc.is_digit(10) {
                return CmpHand::LOWER;
            } else {
                let self_index = CARDS.iter().position(|c| *c == shc).unwrap();
                let other_index = CARDS.iter().position(|c| *c == ohc).unwrap();

                if self_index > other_index {
                    return CmpHand::HIGHER;
                } else if self_index < other_index {
                    return CmpHand::LOWER;
                }
            }
        }

        CmpHand::EQUAL
    }
}

#[derive(Debug)]
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

    fn get_total_winnings(&mut self) -> String {
        self.sort_hands();

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

    fn sort_hands(&mut self) {
        for line in self.lines.clone().iter() {
            let hand_type = &line.hand_type;
            if self.types.contains_key(&hand_type) {
                self.insert_sorted(line.clone(), hand_type.clone());
            } else {
                self.types.insert(hand_type.clone(), vec![line.clone()]);
            }
        }
    }

    fn insert_sorted(&mut self, hand: Line, hand_type: Type) {
        let mut vector: Vec<Line> = self.types.get(&hand_type).unwrap().to_vec();
        let mut left = 0;
        let mut right = vector.len() - 1;
        let mut middle: usize;

        loop {
            middle = (right + left) / 2;
            match hand.compare(vector[middle].hand.clone()) {
                CmpHand::EQUAL => {
                    vector.insert(middle, hand);
                    break;
                }
                CmpHand::LOWER => {
                    if middle == left {
                        vector.insert(middle, hand);
                        break;
                    }

                    right = middle - 1;
                }
                CmpHand::HIGHER => {
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

    fn part_1(parsed_input: &mut Self::ParsedInput) -> String {
        parsed_input.get_total_winnings()
    }

    fn part_2(_parsed_input: &mut Self::ParsedInput) -> String {
        "".to_string()
    }
}
