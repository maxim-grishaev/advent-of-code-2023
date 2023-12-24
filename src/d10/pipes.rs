use super::dir::PipeDirection;
use std::fmt::Debug;
use PipeDirection::*;

pub struct PipeMap {
    pub data: Vec<Vec<char>>,
}

impl Debug for PipeMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for line in &self.data {
            s.push_str(&line.iter().collect::<String>());
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

pub type PipePos = usize;
pub type PipeCoord = (PipePos, PipePos);

// Static
impl PipeMap {
    pub const START: char = 'S';
    pub const PIPE_HOR: char = '-';
    pub const PIPE_VERT: char = '|';
    pub const PIPE_BR: char = 'F';
    pub const PIPE_LB: char = '7';
    pub const PIPE_TR: char = 'L';
    pub const PIPE_LT: char = 'J';

    fn chars_line_to_vec(line: &str) -> Vec<char> {
        line.chars().collect::<Vec<_>>()
    }

    pub fn from_str(str: &str) -> PipeMap {
        let data = str
            .trim()
            .lines()
            .map(PipeMap::chars_line_to_vec)
            .collect::<Vec<_>>();
        PipeMap { data }
    }

    pub fn get_dirs_by_char(c: char) -> Option<(PipeDirection, PipeDirection)> {
        match c {
            PipeMap::PIPE_VERT => Some((PipeDirection::Top, PipeDirection::Bottom)),
            PipeMap::PIPE_HOR => Some((PipeDirection::Left, PipeDirection::Right)),
            PipeMap::PIPE_LB => Some((PipeDirection::Bottom, PipeDirection::Left)),
            PipeMap::PIPE_BR => Some((PipeDirection::Bottom, PipeDirection::Right)),
            PipeMap::PIPE_LT => Some((PipeDirection::Top, PipeDirection::Left)),
            PipeMap::PIPE_TR => Some((PipeDirection::Top, PipeDirection::Right)),
            _ => None,
        }
    }

    pub fn get_dir_out_by_char(dir_in: &PipeDirection, c: char) -> Option<PipeDirection> {
        let (aaa, bbb) = PipeMap::get_dirs_by_char(c)?;
        if aaa == *dir_in {
            Some(bbb)
        } else if bbb == *dir_in {
            Some(aaa)
        } else {
            None
        }
    }
}

impl PipeMap {
    pub fn get_coord_at_dir(&self, coord: &PipeCoord, dir: &PipeDirection) -> Option<PipeCoord> {
        let (x, y) = coord;
        match dir {
            PipeDirection::Top => match y {
                y if *y > 0 => Some((*x, *y - 1)),
                _ => None,
            },
            PipeDirection::Bottom => match y {
                y if y < &self.data.len() => Some((*x, *y + 1)),
                _ => None,
            },
            PipeDirection::Left => match x {
                x if *x > 0 => Some((*x - 1, *y)),
                _ => None,
            },
            PipeDirection::Right => match x {
                x if y < &self.data.len() && x < &self.data[*y].len() => Some((*x + 1, *y)),
                _ => None,
            },
        }
    }

    pub fn get_chat_at(&self, coord: &PipeCoord) -> Option<char> {
        let (x, y) = coord;
        match self.data.get(*y) {
            None => None,
            Some(line) => line.get(*x).copied(),
        }
    }

    pub fn get_start_coord(&self) -> Option<(usize, usize)> {
        for (y, line) in self.data.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if *c == PipeMap::START {
                    return Some((x, y));
                }
            }
        }
        None
    }

    pub fn for_each<F>(&self, mut clbk: F)
    where
        F: FnMut(&char, PipeCoord),
    {
        for (y, line) in self.data.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                clbk(c, (x, y));
            }
        }
    }

    pub fn get_start_joint_char(&self) -> Result<char, String> {
        let start_coord = self.get_start_coord().unwrap();

        let start_dirs: Vec<PipeDirection> = vec![Top, Right, Bottom, Left]
            .into_iter()
            .filter(|d| {
                self.get_coord_at_dir(&start_coord, d)
                    .and_then(|next_coord| {
                        let d_in = PipeDirection::invert(d);
                        let next_char = self.get_chat_at(&next_coord)?;
                        PipeMap::get_dir_out_by_char(&d_in, next_char)
                    })
                    .is_some()
            })
            .collect();

        if start_dirs.len() != 2 {
            Result::Err(format!(
                "Invalid start connection: {:?} @ {:?}",
                start_dirs, start_coord
            ))?;
        }

        let ch = match (start_dirs[0], start_dirs[1]) {
            (Top, Right) => PipeMap::PIPE_TR,    // L
            (Top, Bottom) => PipeMap::PIPE_VERT, // |
            (Top, Left) => PipeMap::PIPE_LT,     // J
            (Right, Bottom) => PipeMap::PIPE_BR, // F
            (Right, Left) => PipeMap::PIPE_HOR,  // -
            (Bottom, Left) => PipeMap::PIPE_LB,  // 7
            _ => panic!("Invalid start connection"),
        };
        Result::Ok(ch)
    }

    pub fn follow_path(
        &self,
        start_coord: &PipeCoord,
        start_dir: &PipeDirection,
    ) -> Option<Vec<PipeCoord>> {
        let second_coord = self.get_coord_at_dir(start_coord, start_dir)?;

        let mut in_dir = start_dir.invert();
        // Test if we can move into second coord
        self.get_chat_at(&second_coord)
            .and_then(|c| PipeMap::get_dir_out_by_char(&in_dir, c))?;

        let mut path = vec![];
        let mut current_coord = second_coord;

        loop {
            let next = self
                .get_chat_at(&current_coord)
                .and_then(|c| PipeMap::get_dir_out_by_char(&in_dir, c))
                .and_then(|out_dir| {
                    self.get_coord_at_dir(&current_coord, &out_dir)
                        .map(|next_coord| {
                            path.push(current_coord);
                            current_coord = next_coord;
                            in_dir = out_dir.invert();
                        })
                });
            if next.is_none() {
                break;
            }
        }

        Some(path)
    }

    pub fn find_all_paths(&self) -> Vec<Vec<PipeCoord>> {
        let start_pos = self.get_start_coord().unwrap();

        [Top, Right, Bottom, Left]
            .iter()
            .filter_map(|d| self.follow_path(&start_pos, d))
            .collect()
    }
}

#[cfg(test)]
pub mod test {
    use super::super::help;
    use super::*;
    use insta::assert_debug_snapshot;

    fn get_all_test_maps() -> (PipeMap, PipeMap, PipeMap, PipeMap) {
        (
            PipeMap::from_str(help::t::LOOP_1_S),
            PipeMap::from_str(help::t::LOOP_1_H),
            PipeMap::from_str(help::t::LOOP_2_S),
            PipeMap::from_str(help::t::LOOP_2_H),
        )
    }

    #[test]
    fn parse() {
        let maps = get_all_test_maps();
        assert_debug_snapshot!(maps.0);
        assert_debug_snapshot!(maps.1);
        assert_debug_snapshot!(maps.2);
        assert_debug_snapshot!(maps.3);
    }

    #[test]
    fn start_pos() {
        let maps = get_all_test_maps();
        assert_eq!(maps.0.get_start_coord(), Some((1, 1)));
        assert_eq!(maps.1.get_start_coord(), Some((1, 1)));
        assert_eq!(maps.2.get_start_coord(), Some((0, 2)));
        assert_eq!(maps.3.get_start_coord(), Some((0, 2)));
    }

    #[test]
    fn get_dir_ok() {
        let ok_potions = [
            (PipeMap::PIPE_VERT, Top, Bottom),
            (PipeMap::PIPE_VERT, Bottom, Top),
            (PipeMap::PIPE_HOR, Left, Right),
            (PipeMap::PIPE_HOR, Right, Left),
            (PipeMap::PIPE_LB, Bottom, Left),
            (PipeMap::PIPE_LB, Left, Bottom),
            (PipeMap::PIPE_BR, Bottom, Right),
            (PipeMap::PIPE_BR, Right, Bottom),
            (PipeMap::PIPE_LT, Top, Left),
            (PipeMap::PIPE_LT, Left, Top),
            (PipeMap::PIPE_TR, Top, Right),
            (PipeMap::PIPE_TR, Right, Top),
        ];

        for (c, d_from, d_to) in &ok_potions {
            assert_eq!(
                PipeMap::get_dir_out_by_char(d_from, *c),
                Some(*d_to),
                "Tested: {:?}",
                (&c, &d_from, &d_to)
            );
        }
    }

    #[test]
    fn get_dir_not_ok() {
        let not_ok_potions = [
            (PipeMap::PIPE_VERT, Left),
            (PipeMap::PIPE_VERT, Right),
            (PipeMap::PIPE_HOR, Top),
            (PipeMap::PIPE_HOR, Bottom),
            (PipeMap::PIPE_LB, Top),
            (PipeMap::PIPE_LB, Right),
            (PipeMap::PIPE_BR, Top),
            (PipeMap::PIPE_BR, Left),
            (PipeMap::PIPE_LT, Bottom),
            (PipeMap::PIPE_LT, Right),
            (PipeMap::PIPE_TR, Bottom),
            (PipeMap::PIPE_TR, Left),
        ];

        for (c, d_from) in not_ok_potions {
            assert_eq!(
                PipeMap::get_dir_out_by_char(&d_from, c),
                None,
                "Tested: {:?}",
                (&c, &d_from)
            );
        }
    }

    #[test]
    fn coord_by_dir() {
        let maps = get_all_test_maps();
        let coord = (1, 1);
        assert_eq!(maps.0.get_coord_at_dir(&coord, &Top), Some((1, 0)));
        assert_eq!(maps.0.get_coord_at_dir(&coord, &Bottom), Some((1, 2)));
        assert_eq!(maps.0.get_coord_at_dir(&coord, &Left), Some((0, 1)));
        assert_eq!(maps.0.get_coord_at_dir(&coord, &Right), Some((2, 1)));
    }

    #[test]
    fn follow_path() {
        let map = get_all_test_maps().0;
        let start = map.get_start_coord().unwrap();
        let path = map.follow_path(&start, &Bottom);
        assert_debug_snapshot!(path);
    }
}
