use super::help;

fn get_vec_as_pairs(seeds: &Vec<help::ID>) -> Vec<(help::ID, help::ID)> {
    let mut ranges: Vec<(help::ID, help::ID)> = vec![];
    let half_len = seeds.len() / 2;
    for i in 0..half_len {
        let r_start = seeds[i * 2];
        let r_len = seeds[i * 2 + 1];
        let tpl = (r_start, r_start + r_len);
        ranges.push(tpl);
    }
    println!("ranges: {:?}", ranges);
    ranges
}

pub fn run() -> help::ID {
    let alm = help::parse_almanac(&help::read_almanac());
    let mut min: u64 = u64::MAX;
    for (s, e) in get_vec_as_pairs(&alm.seeds) {
        println!("range: {:?}..{:?}", s, e);
        for n in s..=e {
            min = min.min(help::get_destination(&n, &alm));
        }
        println!("min: {:?}", min);
    }
    println!("dests: {:?}", min);
    min
}
