use std::collections::HashSet;

use super::{help, map::Map};

fn is_end_node(node: &str) -> bool {
    node.ends_with(Map::END_SUFFIX)
}

pub fn run_str(str: &str) -> u128 {
    let map = Map::from(str);

    let start_nodes = map
        .nodes
        .keys()
        .filter(|k| k.ends_with(Map::START_SUFFIX))
        .collect::<Vec<_>>();
    println!("{:?}", start_nodes);

    let counts = start_nodes
        .iter()
        .map(|k| help::count_steps(str, k, is_end_node))
        .collect::<Vec<_>>();
    println!("{:?}", counts);

    let big_num = counts
        .iter()
        .map(|n| (*n as u128))
        .reduce(|a, b| a * b)
        .unwrap();
    println!("{:?}", big_num);

    let primes = help::get_primes(big_num);
    let primes_set = primes.iter().collect::<HashSet<_>>();
    println!("{:?}", primes_set);

    let prod = primes_set.iter().map(|n| **n).reduce(|a, b| a * b).unwrap();
    println!("{:?}", prod);

    prod
}

pub fn run() -> u128 {
    run_str(&help::read_map())
}

#[cfg(test)]
mod tests {
    use super::*;
    // use insta::assert_debug_snapshot;

    const TEST_MAP_GHOST: &str = r#"
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
    "#;

    #[test]
    fn test_run_ghost() {
        assert_eq!(run_str(TEST_MAP_GHOST.trim()), 6);
    }

    #[test]
    fn test_run_real() {
        assert_eq!(run(), 16342438708751);
    }
}
