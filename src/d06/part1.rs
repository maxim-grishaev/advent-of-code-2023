use super::help;

pub fn run() -> usize {
    let ttbl_s = help::read_timetable();
    let ttbl = help::parse_timetable(&ttbl_s);

    // 4568778
    ttbl.iter()
        .map(|(t, d)| help::find_all_race_times(t, d).len())
        .reduce(|a, b| a * b)
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    // use insta::assert_debug_snapshot;

    #[test]
    fn run_test() {
        assert_eq!(run(), 4568778);
    }
}
