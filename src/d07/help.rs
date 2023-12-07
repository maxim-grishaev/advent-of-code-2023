use super::poker;
use crate::util;

pub fn read_poker_string() -> String {
    util::fs::read_file(file!(), "cards.txt")
}

pub fn parse_poker_string(str: &str) -> Vec<poker::GameItem> {
    str.lines().map(|l| poker::GameItem::new(l)).collect()
}

#[cfg(test)]
pub mod t {
    const STR: &str = r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;

    pub fn get_test() -> String {
        STR.trim().to_string()
    }
}
