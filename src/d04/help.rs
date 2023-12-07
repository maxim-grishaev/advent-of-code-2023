use crate::util;

#[derive(Debug)]
pub struct Scratchcard {
    pub name: String,
    pub win: Vec<u32>,
    pub have: Vec<u32>,
}

fn normalize_numbers(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

pub fn read_cards() -> Vec<Scratchcard> {
    let cards_str = util::fs::read_file(file!(), "cards.txt");
    cards_str
        .lines()
        .map(|s| {
            let mut line_split = s.split(':');
            let name = line_split.next().unwrap().to_string();
            let mut nums_split = line_split.next().unwrap().split("|");
            Scratchcard {
                name,
                win: normalize_numbers(nums_split.next().unwrap()),
                have: normalize_numbers(nums_split.next().unwrap()),
            }
        })
        .collect()
}

pub fn intersection(narr1: &Vec<u32>, narr2: &Vec<u32>) -> Vec<u32> {
    narr1
        .into_iter()
        .filter(|win_num| narr2.contains(win_num))
        .copied()
        .collect()
}
