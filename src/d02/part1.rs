use super::cfg;
use crate::util::fs;

pub fn run() -> u32 {
    fs::read_file(file!(), "games.txt")
        .lines()
        .map(|line| cfg::parse_line(line))
        .filter(|g| g.runs.iter().all(cfg::is_valid_run))
        .map(|g| g.game_no)
        .sum()
}
