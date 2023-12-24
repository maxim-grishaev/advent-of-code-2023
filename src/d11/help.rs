use crate::util;

pub fn read_data() -> String {
    util::fs::read_file(file!(), "stars.txt")
}

pub fn get_pairs(ln: usize) -> Vec<(usize, usize)> {
    let mut pairs = vec![];
    for i in 0..ln {
        for j in i + 1..ln {
            pairs.push((i, j));
        }
    }
    pairs
}

pub fn insert_at_many_pos<T>(new_data: Vec<T>, pos_sorted: &[usize], new_item: &T) -> Vec<T>
where
    T: Clone,
{
    let mut new_data = new_data;
    // pos_sorted.sort();
    for line in pos_sorted.iter().rev() {
        new_data.insert(*line, new_item.clone());
    }
    new_data
}

#[cfg(test)]
pub mod t {
    use super::*;
    // use insta::assert_debug_snapshot;

    pub const EXAMPLE: &str = r#"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
    "#;

    #[ignore = "template"]
    #[test]
    fn smoke() {
        // StarMap::from_str(&read_data());
    }

    #[ignore = "template"]
    #[test]
    fn expand() {
        assert_eq!(read_data(), "");
    }
}
