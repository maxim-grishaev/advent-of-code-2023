use crate::util;

use super::map::Map;

pub fn read_map() -> String {
    util::fs::read_file(file!(), "map.txt")
}

pub fn get_primes(n: u128) -> Vec<u128> {
    let mut primes = Vec::new();
    let mut num = n;
    let mut i = 2;

    while i <= num {
        if num % i == 0 {
            primes.push(i);
            num /= i;
        } else {
            i += 1;
        }
    }
    primes
}

pub fn count_steps<IE>(str: &str, start_node: &str, is_end: IE) -> usize
where
    IE: Fn(&str) -> bool,
{
    let map = Map::from(str);
    let dln = map.dir.len();

    let mut i: usize = 0;
    let mut node = start_node;

    loop {
        let d = map.dir.chars().nth(i % dln).unwrap();

        // println!("{:?}", (i, node, d));
        let (l, r) = map.nodes.get(node).unwrap();

        node = match d {
            Map::LEFT => l,
            Map::RIGHT => r,
            _ => panic!("Unknown direction"),
        };

        i += 1;
        if is_end(node) {
            return i;
        }
    }
}

#[cfg(test)]
pub mod t {
    // use super::*;
    use super::super::map::Map;
    use insta::assert_debug_snapshot;

    pub const MAP_1: &str = r#"
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
    "#;

    pub const MAP_2: &str = r#"
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
    "#;

    //     pub const MAP_GHOST: &str = r#"
    // LR

    // 11A = (11B, XXX)
    // 11B = (XXX, 11Z)
    // 11Z = (11B, XXX)
    // 22A = (22B, XXX)
    // 22B = (22C, 22C)
    // 22C = (22Z, 22Z)
    // 22Z = (22B, 22B)
    // XXX = (XXX, XXX)
    //     "#;

    #[ignore = "snap + hashmap"]
    #[test]
    fn parse() {
        assert_debug_snapshot!(Map::from(MAP_1.trim()));
        assert_debug_snapshot!(Map::from(MAP_2.trim()));
    }

    // #[ignore = "snap + hashmap"]
    // #[test]
    // fn map_1() {
    //     assert_eq!(count_steps(MAP_1.trim()), 2);
    // }

    // #[ignore = "snap + hashmap"]
    // #[test]
    // fn map_2() {
    //     assert_eq!(count_steps(MAP_2.trim()), 6);
    // }
}
