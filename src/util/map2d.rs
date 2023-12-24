#![allow(unused)]

use std::fmt::{Debug, Formatter, Result};

#[derive(Debug)]
pub struct Coord2D {
    pub row: usize,
    pub col: usize,
}

impl Coord2D {
    pub fn new(row: usize, col: usize) -> Coord2D {
        Coord2D { row, col }
    }

    pub fn cell_distance(&self, other: &Coord2D) -> usize {
        (self.col as isize - other.col as isize).unsigned_abs()
            + (self.row as isize - other.row as isize).unsigned_abs()
    }
}

#[derive(Debug)]
pub struct Size {
    pub cols_count: usize,
    pub rows_count: usize,
}

#[derive(Debug)]
pub struct Lines2D {
    pub rows: Vec<usize>,
    pub cols: Vec<usize>,
}

pub type Vec2D<T> = Vec<Vec<T>>;

// #[derive(Debug)]
pub struct Map2D<T> {
    pub raw: Vec2D<T>,
}

const NEW_LINE: &str = "\n";
impl Debug for Map2D<char> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let s = self
            .raw
            .iter()
            .map(|line| line.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join(NEW_LINE);
        write!(f, "Map2D<char>\n{}", s)
    }
}

impl Map2D<char> {
    pub fn from_str(str: &str) -> Map2D<char> {
        Map2D::new(
            str.trim()
                .lines()
                .map(|line| line.chars().collect())
                .collect(),
        )
    }
}

impl<T> Map2D<T> {
    pub fn new(data: Vec<Vec<T>>) -> Map2D<T> {
        Map2D { raw: data }
    }

    pub fn get_val_at(&self, y_row: usize, x_col: usize) -> Option<&T> {
        self.raw.get(y_row).and_then(|line| line.get(x_col))
    }

    pub fn get_val_by_coord(&self, coord: &Coord2D) -> Option<&T> {
        self.get_val_at(coord.row, coord.col)
    }

    pub fn size(&self) -> Size {
        let height = self.raw.len();
        let width = self.raw.get(0).map(|line| line.len()).unwrap_or(0);
        Size {
            rows_count: height,
            cols_count: width,
        }
    }

    pub fn find_coords<F>(&self, is_found: F) -> Vec<Coord2D>
    where
        F: Fn(&T) -> bool,
    {
        let mut coords = vec![];
        self.for_each(|item, coord| {
            if is_found(item) {
                coords.push(coord);
            }
        });
        coords
    }

    pub fn for_each<F>(&self, mut clbk: F)
    where
        F: FnMut(&T, Coord2D),
    {
        for (y, row) in self.raw.iter().enumerate() {
            for (x, item) in row.iter().enumerate() {
                clbk(item, Coord2D { col: x, row: y });
            }
        }
    }

    pub fn find_lines_has_all_of<F>(&self, is_good: F) -> Lines2D
    where
        F: Fn(&T, Coord2D) -> bool,
    {
        let size = self.size();
        find_lines(size.rows_count, size.cols_count, |coord| {
            self.get_val_by_coord(&coord)
                .map_or(false, |c| is_good(c, coord))
        })
    }
}

fn bools_to_indexes(v: Vec<bool>) -> Vec<usize> {
    v.into_iter()
        .enumerate()
        .filter_map(|(idx, b)| match b {
            true => Some(idx),
            false => None,
        })
        .collect()
}

#[allow(clippy::needless_range_loop)]
fn find_lines<F>(rows_count: usize, cols_count: usize, is_found: F) -> Lines2D
where
    F: Fn(Coord2D) -> bool,
{
    let mut rows = vec![true; rows_count];
    let mut cols = vec![true; cols_count];
    for y in 0..rows_count {
        for x in 0..cols_count {
            if !is_found(Coord2D { row: y, col: x }) {
                rows[y] = false;
                cols[x] = false;
            }
        }
    }

    Lines2D {
        rows: bools_to_indexes(rows),
        cols: bools_to_indexes(cols),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_each() {
        let m2d = Map2D::new(vec![vec![1, 2], vec![3, 4]]);
        let mut sum = 0;
        m2d.for_each(|item, _coord| {
            sum += item;
        });
        assert_eq!(sum, 10);
    }

    const TEST_MAP_TINY: &str = r#"
..
1.
"#;
    #[test]
    fn find_lines_tiny() {
        let map2d = Map2D::from_str(TEST_MAP_TINY);
        println!("{:?}", map2d);
        let lines = map2d.find_lines_has_all_of(|c, _| c == &'.');
        assert_eq!(lines.rows, vec![0], "rows");
        assert_eq!(lines.cols, vec![1], "cols");
    }

    const TEST_MAP: &str = r#"
.....
1....
..2..
.....
....3
"#;
    #[test]
    fn find_lines() {
        let map2d = Map2D::from_str(TEST_MAP);
        let lines = map2d.find_lines_has_all_of(|c, _| c == &'.');
        assert_eq!(lines.rows, vec![0, 3], "rows");
        assert_eq!(lines.cols, vec![1, 3], "cols");
    }

    #[test]
    fn get_val_at() {
        let m2d = Map2D::new(vec![vec![1, 2], vec![3, 4]]);
        assert_eq!(m2d.get_val_at(0, 0), Some(&1));
        assert_eq!(m2d.get_val_at(0, 1), Some(&2));
        assert_eq!(m2d.get_val_at(1, 0), Some(&3));
        assert_eq!(m2d.get_val_at(1, 1), Some(&4));
        assert_eq!(m2d.get_val_at(2, 0), None);
        assert_eq!(m2d.get_val_at(0, 2), None);
    }
}
