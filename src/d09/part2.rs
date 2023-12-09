use super::help;

pub fn run_str(str: &str) -> help::Sum {
    let all_lines = help::parse_to_lines(str);
    all_lines
        .iter()
        .map(|line| {
            let tower = help::get_tower(line);
            help::count_prev_by_tower(&tower)
        })
        .sum()
}

pub fn run() -> help::Sum {
    run_str(&help::read_data())
}

#[cfg(test)]
mod tests {
    use super::super::help::t;
    use super::*;
    // use insta::assert_debug_snapshot;

    #[test]
    fn the_run() {
        assert_eq!(run(), 964);
    }

    #[test]
    fn test_run_fake() {
        assert_eq!(run_str(t::EXAMPLE), 2);
    }

    #[test]
    fn test_run_fake2() {
        let nums = help::parse_to_lines(t::EXAMPLE)
            .iter()
            .map(|line| {
                let tower = help::get_tower(line);
                help::count_prev_by_tower(&tower)
            })
            .collect::<Vec<_>>();

        assert_eq!(nums, [-3, 0, 5]);
    }
}
