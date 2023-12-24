use super::help;
use super::pipes::PipeMap;

pub fn run_str(str: &str) -> u128 {
    let lns = PipeMap::from_str(str)
        .find_all_paths()
        .into_iter()
        .map(|path| path.len())
        .collect::<Vec<_>>();

    let max_ln = lns.iter().max().unwrap() + 1;
    (max_ln as f64 / 2.).ceil() as u128
}

pub fn run() -> u128 {
    run_str(&help::read_data())
}

#[cfg(test)]
mod tests {
    use super::super::help::t;
    use super::*;

    #[test]
    fn the_run() {
        assert_eq!(run(), 6923);
    }

    #[test]
    fn test_run_1s() {
        assert_eq!(run_str(t::LOOP_1_S.trim()), 4);
    }

    #[test]
    fn test_run_1h() {
        assert_eq!(run_str(t::LOOP_1_H.trim()), 4);
    }

    #[test]
    fn test_run_2s() {
        assert_eq!(run_str(t::LOOP_2_S.trim()), 8);
    }

    #[test]
    fn test_run_2h() {
        assert_eq!(run_str(t::LOOP_2_H.trim()), 8);
    }
}
