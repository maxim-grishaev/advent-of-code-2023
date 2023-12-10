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
    fn test_run_1_s() {
        assert_eq!(run_str(t::LOOP_1_S.trim()), 0);
    }

    #[ignore = "template"]
    #[test]
    fn test_run_2_s() {
        assert_eq!(run_str(t::LOOP_2_S.trim()), 0);
    }
}
