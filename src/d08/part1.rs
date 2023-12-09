use super::{help, map::Map};

pub fn run_str(str: &str) -> usize {
    help::count_steps(str, Map::START_NODE, |n| n == Map::END_NODE)
}

pub fn run() -> usize {
    run_str(&help::read_map())
}

#[cfg(test)]
mod tests {
    use super::super::help::t;
    use super::*;
    // use insta::assert_debug_snapshot;

    #[test]
    fn test_run() {
        assert_eq!(run(), 19951);
    }

    #[test]
    fn map_1() {
        assert_eq!(run_str(t::MAP_1.trim()), 2);
    }

    #[test]
    fn map_2() {
        assert_eq!(run_str(t::MAP_2.trim()), 6);
    }
}
