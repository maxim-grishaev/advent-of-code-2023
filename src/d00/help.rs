use crate::util;

pub fn read_thing() -> String {
    util::fs::read_file(file!(), ".txt")
}

#[cfg(test)]
pub mod t {
    use super::*;
    // use insta::assert_debug_snapshot;

    pub const STR: &str = r#"

    "#;

    fn get_test() -> String {
        STR.trim().to_string()
    }

    #[ignore = "template"]
    #[test]
    fn test_run() {
        assert_eq!(read_thing(), "");
    }
}
