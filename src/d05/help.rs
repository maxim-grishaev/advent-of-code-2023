use crate::util;

pub type ID = u64;
pub type AlmMapCfgLine = (ID, ID, ID);

#[derive(Debug)]
pub struct AlmMap {
    pub title: String,
    pub config: Vec<AlmMapCfgLine>,
}

#[derive(Debug)]
pub struct Almanac {
    pub seeds: Vec<ID>,
    pub maps: Vec<AlmMap>,
}

pub fn read_almanac() -> String {
    util::fs::read_file(file!(), "almanac.txt")
}

pub fn parse_almanac(alm_str: &str) -> Almanac {
    let mut sections = alm_str.split("\n\n");

    let mut seeds_split = sections
        .next()
        .unwrap() // seeds line: "seeds: 79 14 55 13"
        .split(":");
    seeds_split.next(); // seeds label: "seeds"
    let seeds = seeds_split
        .next()
        .unwrap() // seeds string: "79 14 55 13"
        .split_whitespace()
        .map(|s| s.parse::<ID>().unwrap())
        .collect::<Vec<ID>>();

    let mut maps: Vec<AlmMap> = vec![];
    loop {
        let sect = sections.next();
        if sect.is_none() {
            break;
        }

        let mut sect_iter = sect.unwrap().lines();
        let title = sect_iter.next().unwrap(); // title line: "seed-to-soil map:"
        let config_nums = sect_iter.map(|line| {
            let triplet = line
                .split_whitespace()
                .map(|s| s.parse::<ID>().unwrap())
                .collect::<Vec<ID>>();
            if triplet.len() != 3 {
                panic!("triplet.len() != 3: {} {}", title, line);
            }
            (triplet[0], triplet[1], triplet[2])
        });
        maps.push(AlmMap {
            title: title.to_string(),
            config: config_nums.collect(),
        });
    }
    Almanac { seeds, maps }
}

/// The rest of the almanac contains a list of maps which describe how to convert numbers from a source category into numbers in a destination category.
/// That is, the section that starts with seed-to-soil map: describes how to convert a seed number (the source) to a soil number (the destination).
/// This lets the gardener and his team know which soil to use with which seeds, which water to use with which fertilizer, and so on.
///
/// Rather than list every source number and its corresponding destination number one by one, the maps describe entire ranges of numbers that can be converted.
/// Each line within a map contains three numbers: the destination range start, the source range start, and the range length.
///
/// Consider again the example seed-to-soil map:
/// ```
/// 50 98 2
/// 52 50 48
/// ```
/// The first line has a destination range start of 50, a source range start of 98, and a range length of 2.
/// This line means that the source range starts at 98 and contains two values: 98 and 99.
/// The destination range is the same length, but it starts at 50, so its two values are 50 and 51.
/// With this information, you know that seed number 98 corresponds to soil number 50 and that seed number 99 corresponds to soil number 51.
///
/// The second line means that the source range starts at 50 and contains 48 values: 50, 51, ..., 96, 97.
/// This corresponds to a destination range starting at 52 and also containing 48 values: 52, 53, ..., 98, 99.
/// So, seed number 53 corresponds to soil number 55.
///
/// Any source numbers that aren't mapped correspond to the same destination number.
/// So, seed number 10 corresponds to soil number 10.
pub fn get_dest_by_src(src: &ID, amap: &AlmMap) -> ID {
    // let mut dest = *src;
    let dest = *src;
    for (dest_start, src_start, len) in amap.config.iter() {
        if src_start <= &dest && dest < *src_start + *len {
            // dest = dest_start + val - src_start;
            return dest_start + dest - src_start;
        }
    }
    dest
}

pub fn get_destination(seed: &ID, alm: &Almanac) -> ID {
    let mut val = *seed;
    for map in alm.maps.iter() {
        val = get_dest_by_src(&val, map);
    }
    val
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use super::*;

    const AL_STR: &str = r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
    "#;

    fn get_almanac() -> Almanac {
        parse_almanac(AL_STR.trim())
    }

    #[test]
    fn test_run() {
        assert_debug_snapshot!(get_almanac());
    }

    fn get_destination_path(seed: &ID, alm: &Almanac) -> Vec<ID> {
        let mut val = vec![*seed];
        for map in alm.maps.iter() {
            val.push(get_dest_by_src(val.last().unwrap(), map));
        }
        val
    }

    #[test]
    fn get_destination_test() {
        // Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78, humidity 78, location 82.
        assert_eq!(
            get_destination_path(&79, &get_almanac()),
            [79, 81, 81, 81, 74, 78, 78, 82]
        );
        // Seed 14, soil 14, fertilizer 53, water 49, light 42, temperature 42, humidity 43, location 43.
        assert_eq!(
            get_destination_path(&14, &get_almanac()),
            [14, 14, 53, 49, 42, 42, 43, 43]
        );
        // Seed 55, soil 57, fertilizer 57, water 53, light 46, temperature 82, humidity 82, location 86.
        assert_eq!(
            get_destination_path(&55, &get_almanac()),
            [55, 57, 57, 53, 46, 82, 82, 86]
        );
        // Seed 13, soil 13, fertilizer 52, water 41, light 34, temperature 34, humidity 35, location 35.
        assert_eq!(
            get_destination_path(&13, &get_almanac()),
            [13, 13, 52, 41, 34, 34, 35, 35]
        );
    }
}
