use super::help;

pub fn run_str(_str: &str) -> u128 {
    todo!()
}

pub fn run() -> u128 {
    run_str(&help::read_data())
}

#[cfg(test)]
mod tests {
    use super::super::help::t;
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
        assert_eq!(run_str(t::EXAMPLE.trim()), 0);
    }
}
