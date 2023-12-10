use crate::util;

pub fn read_data() -> String {
    util::fs::read_file(file!(), "pipes.txt")
}

#[cfg(test)]
pub mod t {
    use super::*;
    // use insta::assert_debug_snapshot;

    // 4
    pub const LOOP_1_S: &str = r#"
.....
.S-7.
.|.|.
.L-J.
.....
    "#;
    pub const LOOP_1_H: &str = r#"
-L|F7
7S-7|
L|7||
-L-J|
L|-JF
    "#;

    // 8
    pub const LOOP_2_S: &str = r#"
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
    "#;
    pub const LOOP_2_H: &str = r#"
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
    "#;

    #[ignore = "template"]
    #[test]
    fn test_run() {
        assert_eq!(read_data(), "");
    }
}
