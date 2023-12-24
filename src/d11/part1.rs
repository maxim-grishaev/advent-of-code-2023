use crate::d11::help::get_pairs;

use super::help;
use super::star_map::StarMap;

pub fn run_str(str: &str) -> usize {
    let sm1 = StarMap::from_str(str);
    let sm2 = sm1.expand_space();

    let distances: Vec<usize> = get_pairs(sm2.stars.len())
        .into_iter()
        .map(|(i1, i2)| sm2.stars[i1].cell_distance(&sm2.stars[i2]))
        .collect();
    distances.iter().sum()
}

pub fn run() -> usize {
    run_str(&help::read_data())
}

#[cfg(test)]
mod tests {
    use super::*;
    // use insta::assert_debug_snapshot;

    #[test]
    fn the_run() {
        assert_eq!(run(), 9521776);
    }

    #[test]
    fn test_run() {
        assert_eq!(run_str(help::t::EXAMPLE), 374);
    }
}
