use crate::map2d::{Coord2D, Lines2D};

use super::help::{self, get_pairs};
use super::star_map::StarMap;

pub fn run_str(str: &str) -> usize {
    let sm = StarMap::from_str(str);
    let voids = sm.find_voids();

    let distances: Vec<usize> = get_pairs(sm.stars.len())
        .into_iter()
        .map(|(i1, i2)| {
            let c1 = &sm.stars[i1];
            let c2 = &sm.stars[i2];
            c1.cell_distance(c2) + count_voids_between(&voids, c1, c2) * (1000000 - 1)
        })
        .collect();
    distances.iter().sum()
}

fn count_voids_between(voids: &Lines2D, c1: &Coord2D, c2: &Coord2D) -> usize {
    let mut count = 0;

    let lo_row = &c1.row.min(c2.row);
    let hi_row = &c1.row.max(c2.row);
    for row in voids.rows.iter() {
        if row > lo_row && row < hi_row {
            count += 1;
        }
    }

    let lo_col = &c1.col.min(c2.col);
    let hi_col = &c1.col.max(c2.col);
    for col in voids.cols.iter() {
        if col > lo_col && col < hi_col {
            count += 1;
        }
    }
    count
}

pub fn run() -> usize {
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
    fn test_run() {
        assert_eq!(run_str(t::EXAMPLE.trim()), 0);
    }
}
