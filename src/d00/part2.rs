use super::help;

pub fn run_str(_str: &str) -> usize {
    0
}

pub fn run() -> usize {
    run_str(&help::read_data())
}

#[cfg(test)]
mod tests {
    use super::*;
    // use insta::assert_debug_snapshot;

    #[ignore = "template"]
    #[test]
    fn the_run() {
        assert_eq!(run(), 123);
    }

    #[ignore = "template"]
    #[test]
    fn test_run() {
        assert_eq!(run_str(help::t::EXAMPLE), 0);
    }
}
