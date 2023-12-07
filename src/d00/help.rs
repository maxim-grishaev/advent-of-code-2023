use crate::util;

pub fn read_thing() -> String {
    util::fs::read_file(file!(), ".txt")
}

pub fn parse_thing(str: String) -> u32 {
    0
}

#[cfg(test)]
pub mod t {
    // use insta::assert_debug_snapshot;

    use super::*;

    pub const STR: &str = r#"

    "#;

    fn get_test() -> String {
        STR.trim().to_string()
    }

    #[test]
    fn test_run() {
        assert_eq!(parse_thing(get_thing()), None);
    }
}
