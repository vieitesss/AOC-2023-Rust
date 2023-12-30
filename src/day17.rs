#![allow(dead_code)]
#![allow(unused_variables)]

use std::{
    collections::{BinaryHeap, HashMap},
    fmt::Display,
};

use crate::Solution;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn values(&self) -> (isize, isize) {
        match self {
            Dir::Up => (-1, 0),
            Dir::Right => (0, 1),
            Dir::Down => (1, 0),
            Dir::Left => (0, -1),
        }
    }

    fn next_dirs(&self) -> Vec<Dir> {
        match self {
            Dir::Up => vec![Dir::Right, Dir::Up, Dir::Left],
            Dir::Right => vec![Dir::Right, Dir::Down, Dir::Up],
            Dir::Down => vec![Dir::Down, Dir::Right, Dir::Left],
            Dir::Left => vec![Dir::Down, Dir::Up, Dir::Left],
        }
    }

    fn dir_by_index(index: usize) -> Self {
        match index {
            0 => Dir::Up,
            1 => Dir::Right,
            2 => Dir::Down,
            3 => Dir::Left,
            _ => panic!("invalid index {index}"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct State {
    heat: usize,
    point: (isize, isize),
    steps: Vec<usize>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .heat
            .cmp(&self.heat)
            .then_with(|| self.point.cmp(&other.point))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Path(HashMap<(isize, isize), Option<(isize, isize)>>);

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut to_sort = self.0.iter().collect::<Vec<_>>();
        to_sort.sort_by(|a, b| a.1.cmp(b.1));

        for (k, v) in to_sort.iter() {
            writeln!(f, "{k:?} -> {v:?}")?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Matrix(Vec<Vec<usize>>);

impl Matrix {
    fn out_of_bounds(&self, point: &(isize, isize)) -> bool {
        point.0 < 0
            || point.1 < 0
            || point.0 as usize > self.0.len() - 1
            || point.1 as usize > self.0.len() - 1
    }

    fn dijkstra(&self, goal: (isize, isize)) -> Option<usize> {
        let mut current_heat: Vec<Vec<_>> = (0..self.0.len())
            .map(|_| (0..self.0.len()).map(|_| usize::MAX).collect())
            .collect();

        let mut frontier: BinaryHeap<State> = BinaryHeap::new();
        let mut came_from = Path(HashMap::new());
        came_from.0.insert((0, 0), None);

        current_heat[0][0] = 0;
        frontier.push(State {
            heat: 0,
            point: (0, 0),
            steps: vec![0; 4],
        });

        while let Some(State { heat, point, steps }) = frontier.pop() {
            if point == goal {
                let mut current = goal;
                let mut path: Vec<(isize, isize)> = vec![];
                while current != (0, 0) {
                    path.push(current);
                    if let Some(new_cur) = came_from.0.get(&current) {
                        current = new_cur.unwrap();
                    } else {
                        break;
                    }
                }
                path.push((0, 0));

                for p in path.iter().rev() {
                    eprintln!("{p:?}");
                }

                return Some(heat);
            }

            // hay otro camino mejor
            if heat > current_heat[point.0 as usize][point.1 as usize] {
                continue;
            }

            // 0, 1, 2, 3
            if let Some(prev_dir) = steps.iter().position(|step| *step > 0) {
                // eprintln!("{prev_dir}");
                for n_dir in Dir::dir_by_index(prev_dir).next_dirs().iter() {
                    // eprintln!("  check {n_dir:?}");

                    if point.0 > 3
                        && point.1 > 3
                        && point.0 < (self.0.len() - 3) as isize
                        && point.1 < (self.0.len() - 3) as isize
                    {
                        continue;
                    }

                    if ((point.0 < 5 && point.1 < (self.0.len() - 5) as isize)
                        || (point.0 > (self.0.len() - 5) as isize && point.1 < 4))
                        && *n_dir == Dir::Left
                    {
                        continue;
                    }

                    let next_point = (point.0 + n_dir.values().0, point.1 + n_dir.values().1);

                    if self.out_of_bounds(&next_point) {
                        continue;
                    }

                    let next_steps: Vec<_> = steps
                        .iter()
                        .enumerate()
                        .map(|(i, step)| {
                            if i == (*n_dir as usize) {
                                return *step + 1;
                            }
                            0
                        })
                        .collect();

                    // no se puede seguir por esa direccion
                    if next_steps.iter().any(|step| *step > 3) {
                        // eprintln!("too many steps");
                        continue;
                    }

                    let next_state = State {
                        heat: heat + self.0[next_point.0 as usize][next_point.1 as usize],
                        point: next_point,
                        steps: next_steps,
                    };

                    // eprintln!("{point:?} -> {:?}", next_state);

                    if next_state.heat <= current_heat[next_point.0 as usize][next_point.1 as usize]
                    {
                        // eprintln!("Added");
                        frontier.push(next_state.clone());
                        current_heat[next_point.0 as usize][next_point.1 as usize] =
                            next_state.heat;
                        came_from.0.insert(next_point, Some(point));
                    } else {
                        // eprintln!("Not added");
                    }
                }
            } else {
                // right
                frontier.push(State {
                    heat: self.0[0][1],
                    point: (0, 1),
                    steps: vec![0, 1, 0, 0],
                });
                current_heat[0][1] = self.0[0][1];
                // down
                frontier.push(State {
                    heat: self.0[1][0],
                    point: (1, 0),
                    steps: vec![0, 0, 1, 0],
                });
                current_heat[1][0] = self.0[1][0];
            }
        }

        None
    }
}

pub struct Day17;

impl Solution for Day17 {
    type ParsedInput = Matrix;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        Matrix(
            input_lines
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap() as usize)
                        .collect()
                })
                .collect(),
        )
    }

    fn part_1(parsed_input: &Self::ParsedInput) -> String {
        if let Some(res) = parsed_input.dijkstra((
            (parsed_input.0.len() - 1) as isize,
            (parsed_input.0.len() - 1) as isize,
        )) {
            return res.to_string();
        }

        String::from("None")
    }

    fn part_2(parsed_input: Self::ParsedInput) -> String {
        "".to_string()
    }
}
