use std::collections::HashMap;

use crate::Solution;

pub struct Day15;

#[derive(Debug, PartialEq)]
enum OP {
    ADD,
    REMOVE,
}

#[derive(Debug)]
struct Step {
    def: String,
    label: String,
    operation: OP,
    value: Option<usize>,
}

impl Step {
    fn get_value(&self) -> usize {
        Step::apply_hash(&self.def)
    }

    fn get_box(&self) -> usize {
        Step::apply_hash(&self.label)
    }

    fn get_id(&self) -> (String, Option<usize>) {
        (self.label.clone(), self.value)
    }

    fn apply_hash(text: &str) -> usize {
        text.chars().fold(0, |mut cur, c| {
            cur += c as usize;
            cur *= 17;
            cur %= 256;
            cur
        })
    }
}

#[derive(Debug)]
pub struct Steps(Vec<Step>);

impl Steps {
    fn get_total_sum(&self) -> usize {
        self.0.iter().map(|step| step.get_value()).sum()
    }
}

type Box = Vec<(String, Option<usize>)>;

impl Solution for Day15 {
    type ParsedInput = Steps;

    fn parse_input(mut input_lines: &str) -> Self::ParsedInput {
        input_lines = input_lines.trim();
        Steps(
            input_lines
                .split(",")
                .map(|step| {
                    if step.contains("=") {
                        let splited: Vec<&str> = step.split("=").collect();
                        return Step {
                            def: step.trim().to_string(),
                            label: splited[0].to_string(),
                            operation: OP::ADD,
                            value: Some(splited[1].parse().unwrap()),
                        };
                    }

                    Step {
                        def: step.trim().to_string(),
                        label: step.split_at(step.len() - 1).0.to_string(),
                        operation: OP::REMOVE,
                        value: None,
                    }
                })
                .collect(),
        )
    }

    fn part_1(parsed_input: &Self::ParsedInput) -> String {
        parsed_input.get_total_sum().to_string()
    }

    fn part_2(parsed_input: Self::ParsedInput) -> String {
        let mut boxes: HashMap<usize, Box> = HashMap::new();
        parsed_input.0.iter().for_each(|step| {
            let op = &step.operation;
            let b = step.get_box();

            if let Some(boxx) = boxes.get(&b) {
                match op {
                    OP::ADD => {
                        let mut new_box = boxx.clone();
                        if let Some(position) = boxx.iter().position(|id| id.0 == step.label) {
                            new_box[position] = step.get_id();
                        } else {
                            new_box.push(step.get_id());
                        }
                        boxes.insert(b, new_box);
                    }
                    OP::REMOVE => {
                        boxes.insert(
                            b,
                            boxx.clone()
                                .extract_if(|id| id.0 != step.get_id().0)
                                .collect(),
                        );
                    }
                }
            } else if *op == OP::ADD {
                boxes.insert(b, vec![step.get_id()]);
            }
        });

        let mut total = 0;
        for (n, boxx) in boxes.iter() {
            for (slot, lens) in boxx.iter().enumerate() {
                total += (n + 1) * (slot + 1) * lens.1.unwrap();
            }
        }

        total.to_string()
    }
}
