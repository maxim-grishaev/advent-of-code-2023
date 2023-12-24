use crate::util;

pub fn read_data() -> String {
    util::fs::read_file(file!(), "springs.txt")
}

#[cfg(test)]
pub mod t {
    use super::*;
    // use insta::assert_debug_snapshot;

    pub const TEST_LINES: [(&str, usize); 6] = [
        ("???.### 1,1,3", 1),
        (".??..??...?##. 1,1,3", 4),
        ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
        ("????.#...#... 4,1,1", 1),
        ("????.######..#####. 1,6,5", 4),
        ("?###???????? 3,2,1 ", 10),
    ];

    pub fn get_test_string() -> String {
        TEST_LINES
            .iter()
            .map(|(s, _)| s.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[ignore = "template"]
    #[test]
    fn test_run() {
        assert_eq!(read_data(), "");
    }
}
