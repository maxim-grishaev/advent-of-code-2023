use std::fmt::Debug;

use crate::util;

pub type Num = i64;
pub type Sum = i128;

#[derive(Clone)]
pub struct Line(Vec<Num>);

impl Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let nums = self
            .0
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        write!(f, "{}", nums)
    }
}

impl Line {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn iter(&self) -> impl Iterator<Item = &Num> {
        self.0.iter()
    }

    fn get(&self, i: usize) -> Option<&Num> {
        self.0.get(i)
    }
}

pub fn read_data() -> String {
    util::fs::read_file(file!(), "readings.txt")
}

pub fn parse_line(str: &str) -> Line {
    Line(str.split_whitespace().map(|n| n.parse().unwrap()).collect())
}

pub fn parse_to_lines(str: &str) -> Vec<Line> {
    str.trim().lines().map(parse_line).collect()
}

pub fn get_diff_line(line: &Line) -> Line {
    let dln = line.len() - 1;
    let mut diffs = vec![0; dln];
    for i in 0..dln {
        diffs[i] = line.get(i + 1).unwrap() - line.get(i).unwrap();
    }
    Line(diffs)
}

pub fn are_zeros(line: &Line) -> bool {
    line.iter().all(|&n| n == 0)
}

pub fn get_tower(line: &Line) -> Vec<Line> {
    let mut diffs: Vec<Line> = vec![];
    let mut cur_line = line.clone();

    while !are_zeros(&cur_line) {
        diffs.push(cur_line.clone());
        cur_line = get_diff_line(&cur_line);
    }

    diffs
}

pub fn count_prev_by_tower(tower: &Vec<Line>) -> Sum {
    let mut count: Sum = 0;
    for line in tower.iter().rev() {
        let first_n = *line.iter().nth(0).unwrap();
        count = first_n as Sum - count;
    }

    count
}

pub fn count_next_by_tower(tower: &Vec<Line>) -> Sum {
    let mut count: Sum = 0;
    for line in tower.iter().rev() {
        let last_n = *line.iter().last().unwrap();
        count += last_n as Sum;
    }

    count
}

#[cfg(test)]
pub mod t {
    use super::*;
    use insta::assert_debug_snapshot;

    pub const EXAMPLE: &str = r#"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
    "#;

    #[test]
    fn test_run() {
        assert_debug_snapshot!(parse_to_lines(EXAMPLE.trim()));
    }

    #[test]
    fn test_run_diffs() {
        let nums = parse_to_lines(EXAMPLE.trim());
        assert_debug_snapshot!(nums.iter().map(get_tower).collect::<Vec<_>>());
    }
}
