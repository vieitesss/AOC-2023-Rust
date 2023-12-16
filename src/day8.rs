use std::collections::HashMap;

use regex::Regex;

use crate::Solution;

pub struct Day8;

#[derive(Debug)]
pub struct Game {
    instructions: Vec<usize>,
    nodes: HashMap<String, Vec<String>>,
}

impl Game {
    // For part 2
    fn get_start_points(&self) -> Vec<String> {
        // self.nodes
        //     .iter()
        //     .filter(|n| n.name.ends_with("A"))
        //     .map(|n| n.clone())
        //     .collect()
        vec![]
    }

    fn get_steps(&self, node_name: String, ends_z: bool) -> usize {
        let mut steps = 0;

        let mut current_node = node_name.clone();
        while !self.is_final_node(current_node.clone(), ends_z) {
            current_node = self.nodes.get(&current_node).unwrap()
                [self.instructions[steps % self.instructions.len()]]
            .clone();
            steps += 1;
        }

        steps
    }

    fn is_final_node(&self, node_name: String, ends_z: bool) -> bool {
        if ends_z {
            return node_name.ends_with("Z");
        }

        node_name == "ZZZ"
    }
}

impl Solution for Day8 {
    type ParsedInput = Game;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let instructions = input_lines
            .lines()
            .nth(0)
            .unwrap()
            .chars()
            .map(|c| if c == 'R' { 1 } else { 0 })
            .collect::<Vec<usize>>();

        let mut nodes: HashMap<String, Vec<String>> = HashMap::new();
        let re = Regex::new(r"([A-Z]{3})\s=\s\(([A-Z]{3}),\s([A-Z]{3})\)").unwrap();

        for (_, [name, left, right]) in re.captures_iter(input_lines).map(|c| c.extract()) {
            nodes.insert(name.to_string(), vec![left.to_string(), right.to_string()]);
        }

        Game {
            instructions,
            nodes,
        }
    }

    fn part_1(parsed_input: &mut Self::ParsedInput) -> String {
        // println!("{:#?}", parsed_input);
        parsed_input.get_steps("AAA".to_string(), false).to_string()
    }

    fn part_2(_parsed_input: &mut Self::ParsedInput) -> String {
        "".to_string()
    }
}
