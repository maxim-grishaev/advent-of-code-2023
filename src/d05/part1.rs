use super::help;

pub fn run() -> help::ID {
    let alm = help::parse_almanac(&help::read_almanac());
    let dests: Vec<help::ID> = alm
        .seeds
        .iter()
        .map(|seed| help::get_destination(seed, &alm))
        .collect();
    // 324724204
    *dests.iter().min().unwrap()
}
