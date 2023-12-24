use super::help::insert_at_many_pos;
use crate::map2d::{Coord2D, Lines2D, Map2D};

#[derive(Debug)]
pub struct StarMap {
    pub stars: Vec<Coord2D>,
    pub map2d: Map2D<char>,
}

impl StarMap {
    const STAR: char = '#';
    const EMPTY: char = '.';
    pub fn from_map2d(m2d: Map2D<char>) -> StarMap {
        let stars = m2d.find_coords(|c| c == &Self::STAR);
        StarMap { stars, map2d: m2d }
    }

    pub fn from_str(str: &str) -> StarMap {
        StarMap::from_map2d(Map2D::from_str(str))
    }
}

impl StarMap {
    pub fn find_voids(&self) -> Lines2D {
        self.map2d.find_lines_has_all_of(|c, _| c == &Self::EMPTY)
    }

    pub fn expand_space(&self) -> StarMap {
        let lines = self.find_voids();
        let size = self.map2d.size();
        let empty_line = vec![Self::EMPTY; size.cols_count];
        let new_data = self.map2d.raw.clone();
        let new_data = insert_at_many_pos(new_data, &lines.rows, &empty_line);
        let new_data = new_data
            .into_iter()
            .map(|line| insert_at_many_pos(line, &lines.cols, &Self::EMPTY))
            .collect();
        let m2d = Map2D::new(new_data);
        StarMap::from_map2d(m2d)
    }
}

// const x: &'static str = r#"
// 1....
// ..2..
// ....3
// "#;
// 0021-3
// 0042-6
