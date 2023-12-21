#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt::Display;

use crate::Solution;

pub struct Day12;

#[derive(Debug)]
struct Record {
    condition: String,
    groups: Vec<usize>,
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.condition, self.groups)
    }
}

impl Record {
    fn find_arrangements(&self, condi: usize, gri: usize, count: usize) -> usize {
        let cond = self.condition.chars().nth(condi);

        // Ya he colocado todos los grupos teniendo en cuenta las condiciones, pero no he terminado
        // de revisar toda la linea. Igualmente es una opcion valida.
        if gri == self.groups.len() && !self.condition.split_at(condi).1.chars().any(|c| c == '#') {
            //println!("correct");
            return 1;
        } else if gri == self.groups.len() {
            //println!("false");
            return 0;
        }

        let n = self.groups[gri]; // cantidad de '#' seguidas que debe haber

        //println!("{condi}->{cond:?} {gri}->{n} {count}");

        // He terminado de mirar la linea de condiciones y el ultimo caracter conincide con el
        // final del ultimo grupo, por lo tanto es una opcion valida.
        if cond.is_none() && n == count && gri == self.groups.len() - 1 {
            //println!("correct");
            return 1;
        } else if cond.is_none() {
            // He terminado de mirar la linea pero no he terminado de
            // colocar todos los grupos
            //println!("false");
            return 0;
        }

        match cond.unwrap() {
            '?' => {
                if n == count {
                    // he terminado el grupo anterior y puedo poner un punto
                    return self.find_arrangements(condi + 1, gri + 1, 0);
                } else if count > 0 {
                    // puedo seguir poniendo '#', no he terminado el grupo
                    return self.find_arrangements(condi + 1, gri, count + 1);
                }
                // count == 0, puedo poner tanto un '#' como un '.'
                return self.find_arrangements(condi + 1, gri, count + 1)
                    + self.find_arrangements(condi + 1, gri, count);
            }
            '.' => {
                if n == count {
                    // ya se ha colocado el grupo
                    return self.find_arrangements(condi + 1, gri + 1, 0);
                } else if count > 0 {
                    //println!("false");
                    return 0;
                } else {
                    // count == 0
                    return self.find_arrangements(condi + 1, gri, count);
                }
            }
            '#' => {
                if n == count {
                    // no deberia haber otro '#'
                    //println!("false");
                    return 0;
                } else {
                    // count >= 0
                    return self.find_arrangements(condi + 1, gri, count + 1);
                }
            }
            _ => panic!("Not a symbol"),
        }
    }
}

#[derive(Debug)]
pub struct Records(Vec<Record>);

impl Records {
    fn sum_of_arrangements(&self) -> usize {
        self.0
            .iter()
            .fold(0, |cur, r| cur + r.find_arrangements(0, 0, 0))
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
        "".to_string()
    }
}

