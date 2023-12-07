pub type Time = u32; // keep them separate
pub type Distance = u64; // just in case
pub type Race = (Time, Distance);
pub type Timetable = Vec<Race>;

const TIMETABLE: &str = r#"
Time:        48     98     90     83
Distance:   390   1103   1112   1360
"#;
//4_294_967_295
// const x: (Time, Distance) = (48_989_083, 390_110_311_121_360);

pub fn read_timetable() -> String {
    TIMETABLE.trim().to_string()
}

fn read_line<T>(line_opt: Option<&str>) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    line_opt
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<T>().unwrap())
        .collect()
}

pub fn parse_timetable(timetable: &str) -> Timetable {
    let mut lines = timetable.lines();
    let times = read_line::<Time>(lines.next());
    let distances = read_line::<Distance>(lines.next());
    times
        .into_iter()
        .zip(distances.into_iter())
        .collect::<Timetable>()
}

fn read_line_as_one_number(line_opt: Option<&str>) -> String {
    // println!("{:?}", line_opt);
    line_opt
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join("")
}

pub fn parse_as_one_race(timetable: &str) -> Race {
    let mut lines = timetable.lines();
    let time = read_line_as_one_number(lines.next())
        .parse::<Time>()
        .unwrap();
    let distance = read_line_as_one_number(lines.next())
        .parse::<Distance>()
        .unwrap();
    (time, distance)
}

fn get_distance_by_charge_time(charge_time: Time, race_time: Time) -> Distance {
    Distance::saturating_mul(
        charge_time as Distance,
        (race_time - charge_time) as Distance,
    )
}

pub fn find_all_race_times(race_time: &Time, record_distance: &Distance) -> Vec<Distance> {
    let mut res: Vec<Distance> = vec![];
    println!(
        "race_time: {:?}, record_distance: {:?}",
        race_time, record_distance
    );
    let mut max_d: Distance = 0;
    for time in 1..*race_time {
        let d = get_distance_by_charge_time(time, *race_time);
        if d > max_d {
            max_d = d;
        }
        if d > *record_distance {
            res.push(d);
        }
    }
    println!("max: {:?}, {:?}", max_d, record_distance);
    println!("tot: {:?}", res.len());
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;

    const TIMETABLE_TEST: &str = r#"
Time:      7  15   30
Distance:  9  40  200
    "#;

    fn get_ttbl() -> String {
        TIMETABLE_TEST.trim().to_string()
    }

    #[test]
    fn smoke() {
        let timetable = get_ttbl();
        let parsed = parse_timetable(&timetable);
        assert_debug_snapshot!(parsed);
    }

    #[test]
    fn get_times() {
        let timetable = get_ttbl();
        let parsed = parse_timetable(&timetable);
        assert_debug_snapshot!(find_all_race_times(&parsed[0].0, &parsed[0].1));
        assert_debug_snapshot!(find_all_race_times(&parsed[1].0, &parsed[1].1));
        assert_debug_snapshot!(find_all_race_times(&parsed[2].0, &parsed[2].1));
    }

    #[test]
    fn parse_as_one_race_smoke() {
        let timetable = get_ttbl();
        assert_eq!(parse_as_one_race(&timetable), (71530, 940200));
    }

    #[test]
    fn parse_as_one_race_count_times() {
        let ttbl_s = get_ttbl();
        let race = parse_as_one_race(&ttbl_s);
        // println!("{:?}", race); // (71530, 940200)
        assert_eq!(find_all_race_times(&race.0, &race.1).len(), 71_503);
    }
}
