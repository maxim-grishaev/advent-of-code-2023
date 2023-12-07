use super::help;

pub fn run() -> usize {
    let ttbl_s = help::read_timetable();
    let race = help::parse_as_one_race(&ttbl_s);
    println!("{:?}", race); // (48989083, 390110311121360)
    help::find_all_race_times(&race.0, &race.1).len()
}

#[cfg(test)]
mod test {
    use super::*;
    // use insta::assert_debug_snapshot;

    #[test]
    fn run_test() {
        assert_eq!(run(), 28_973_936);
    }
}
