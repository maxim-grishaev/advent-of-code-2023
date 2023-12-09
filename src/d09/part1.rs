use super::help;

pub fn run_str(str: &str) -> help::Sum {
    let all_lines = help::parse_to_lines(str);
    all_lines
        .iter()
        .map(|line| {
            let tower = help::get_tower(line);
            help::count_next_by_tower(&tower)
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

    #[test]
    fn the_run() {
        assert_eq!(run(), 1904165718);
    }

    #[test]
    fn test_run_fake() {
        assert_eq!(run_str(t::EXAMPLE), 114);
    }
}
