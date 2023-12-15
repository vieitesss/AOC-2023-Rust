#![allow(dead_code)]

use regex::Regex;

use crate::Solution;

pub struct Day8;

#[derive(Debug)]
pub struct Game {
    instructions: String,
    nodes: Vec<Node>,
}

impl Game {
    // For part 2
    fn get_start_points(&self) -> Vec<Node> {
        self.nodes
            .iter()
            .filter(|n| n.name.ends_with("A"))
            .map(|n| n.clone())
            .collect()
    }

    fn get_steps(&self, node_name: String, ends_z: bool) -> usize {
        let mut steps = 0;
        let mut index = 0;
        let mut node = self.find_node(node_name);
        loop {
            let next_ins = self.instructions.chars().nth(index).unwrap();
            let next_node = if next_ins == 'R' {
                self.find_node(node.right.clone())
            } else {
                self.find_node(node.left.clone())
            };
            if next_node.is_final_node(next_node.name.clone(), ends_z) {
                return steps + 1;
            }

            steps += 1;
            index = (index + 1) % self.instructions.len();
            node = next_node
        }
    }

    fn find_node(&self, node_name: String) -> Node {
        let mut left = 0;
        let mut right = self.nodes.len() - 1;
        let mut middle: usize;

        loop {
            middle = (left + right) / 2;

            let node = self.nodes.iter().nth(middle).unwrap();
            match node.name.cmp(&node_name) {
                std::cmp::Ordering::Less => {
                    left = middle + 1;
                }

                std::cmp::Ordering::Greater => {
                    right = middle - 1;
                }
                std::cmp::Ordering::Equal => {
                    return node.clone()
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn is_final_node(&self, name: String, ends_z: bool) -> bool {
        return name == "ZZZ" || (ends_z && name.ends_with("Z"));
    }
}

impl Solution for Day8 {
    type ParsedInput = Game;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let instructions = input_lines.lines().nth(0).unwrap().to_string();
        let mut nodes: Vec<Node> = vec![];
        let re = Regex::new(r"([A-Z]{3})\s=\s\(([A-Z]{3}),\s([A-Z]{3})\)").unwrap();

        for (_, [name, left, right]) in re.captures_iter(input_lines).map(|c| c.extract()) {
            nodes.push(Node {
                name: name.to_string(),
                left: left.to_string(),
                right: right.to_string(),
            })
        }

        nodes.sort_by(|a, b| a.name.cmp(&b.name));

        Game {
            instructions,
            nodes,
        }
    }

    fn part_1(parsed_input: &mut Self::ParsedInput) -> String {
        parsed_input.get_steps("AAA".to_string(), false).to_string()
    }

    fn part_2(_parsed_input: &mut Self::ParsedInput) -> String {
        "".to_string()
    }
}

