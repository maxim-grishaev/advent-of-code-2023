use super::cfg;
use crate::util::fs;
use std::cmp::max;

fn calc_game_score(g: cfg::GameResult) -> u32 {
    let mut rgb: [u32; 3] = [0, 0, 0];
    g.runs.iter().for_each(|hm| {
        rgb = [
            max(rgb[0], *hm.get("red").unwrap_or(&0)),
            max(rgb[1], *hm.get("green").unwrap_or(&0)),
            max(rgb[2], *hm.get("blue").unwrap_or(&0)),
        ]
    });
    let pwr = rgb[0] * rgb[1] * rgb[2];
    // println!("{} => {:?} => {:?}", g.game_no, rgb, pwr);
    pwr
}

pub fn run() -> u32 {
    fs::read_file(file!(), "games.txt")
        .lines()
        .map(|line| cfg::parse_line(line))
        .map(calc_game_score)
        .sum()
}
