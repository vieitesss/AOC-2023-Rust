use crate::Solution;

pub struct Day15;

#[derive(Debug)]
struct Step(String);
#[derive(Debug)]
struct Step {
    def: String,
    label: String,
    operation: OP,
    value: Option<usize>,
}

impl Step {
    fn get_value(&self) -> usize {
        let mut total = 0;
        for ch in self.0.chars() {
            total += ch as usize;
            total *= 17;
            total %= 256;
        }

        total
    }
}

#[derive(Debug)]
pub struct Steps(Vec<Step>);

impl Steps {
    fn get_total_sum(&self) -> usize {
        self.0.iter().map(|step| step.get_value()).sum()
    }
}

impl Solution for Day15 {
    type ParsedInput = Steps;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
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

    fn part_2(_parsed_input: Self::ParsedInput) -> String {
        "".to_string()
    }
}
