use crate::Solution;

pub struct Day6;

pub struct Paper(Vec<Race>);

impl Paper {
    fn ways_to_beat_record(&self) -> String {
        self.0
            .iter()
            .fold(1, |cur, r| cur * r.get_ways_to_win())
            .to_string()
    }

    fn ways_to_beat_race(&self) -> String {
        let race = self.get_single_race();
        race.get_ways_to_win().to_string()
    }

    fn get_single_race(&self) -> Race {
        let mut time_str = "".to_string();
        let mut record_str = "".to_string();
        for i in 0..self.0.len() {
            time_str = format!("{}", time_str + &self.0[i].time.to_string());
            record_str = format!("{}", record_str + &self.0[i].record.to_string());
        }

        let time = time_str.parse().unwrap();
        let record = record_str.parse().unwrap();

        Race { time, record }
    }
}

#[derive(Debug)]
struct Race {
    time: usize,
    record: usize,
}

impl Race {
    fn get_ways_to_win(&self) -> usize {
        let time = self.get_min_time();

        self.time - time - time + 1
    }

    fn get_min_time(&self) -> usize {
        let time = self.time as f32;
        let distance = self.record as f32;
        ((-(time) + ((time * time + 4f32 * (-distance)) as f32).sqrt()) / (-2f32)) as usize + 1
    }
}

impl Solution for Day6 {
    type ParsedInput = Paper;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let mut lines = input_lines.lines();
        let times_str: &str = lines.next().unwrap();
        let distances_str: &str = lines.next().unwrap();

        let times: Vec<usize> = times_str
            .split(':')
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|t| t.parse().unwrap())
            .collect();
        let distances: Vec<usize> = distances_str
            .split(':')
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|t| t.parse().unwrap())
            .collect();

        let mut races = vec![];
        for i in 0..times.len() {
            races.push(Race {
                time: times[i],
                record: distances[i],
            })
        }

        Paper(races)
    }

    fn part_1(parsed_input: &mut Self::ParsedInput) -> String {
        parsed_input.ways_to_beat_record()
    }

    fn part_2(parsed_input: &mut Self::ParsedInput) -> String {
        parsed_input.ways_to_beat_race()
    }
}

