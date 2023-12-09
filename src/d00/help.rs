use crate::util;

pub fn read_data() -> String {
    util::fs::read_file(file!(), ".txt")
}

#[cfg(test)]
pub mod t {
    use super::*;
    // use insta::assert_debug_snapshot;

    pub const EXAMPLE: &str = r#"

    "#;

    #[ignore = "template"]
    #[test]
    fn test_run() {
        assert_eq!(read_data(), "");
    }
}
