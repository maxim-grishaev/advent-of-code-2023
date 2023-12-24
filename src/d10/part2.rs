use crate::d10::pipes::PipeCoord;

use super::help;
use super::pipes::PipeMap;

fn get_loop_path(map: &PipeMap) -> Vec<PipeCoord> {
    let start_coord = map.get_start_coord().unwrap();
    let all_paths = map.find_all_paths();
    let mut path: Vec<PipeCoord> = all_paths[0].clone();
    path.push(start_coord);
    path
}

pub fn run_str(str: &str) -> usize {
    let map = PipeMap::from_str(str);
    let path = get_loop_path(&map);
    let mut is_inside_of_the_loop = false;
    let mut prev_pipe_char: Option<char> = None;
    let mut chars_inside_of_the_loop = vec![];

    map.for_each(|char, coord| {
        let is_path = path.contains(&coord);
        if is_inside_of_the_loop && !is_path {
            chars_inside_of_the_loop.push(*char);
            return;
        }

        if !is_path {
            return;
        }

        // replace S with a joint
        let char = match *char {
            PipeMap::START => map.get_start_joint_char().unwrap(),
            _ => *char,
        };

        let has_crossed_border = match char {
            // |
            PipeMap::PIPE_VERT => true,
            // L---7
            PipeMap::PIPE_LB => Some(PipeMap::PIPE_TR) == prev_pipe_char,
            // F---J
            PipeMap::PIPE_LT => Some(PipeMap::PIPE_BR) == prev_pipe_char,
            // L---J or F---7 Didn't cross the border
            _ => false,
        };

        if has_crossed_border {
            is_inside_of_the_loop = !is_inside_of_the_loop;
        }

        prev_pipe_char = match char {
            PipeMap::PIPE_TR => Some(PipeMap::PIPE_TR),
            PipeMap::PIPE_BR => Some(PipeMap::PIPE_BR),
            PipeMap::PIPE_VERT => None,
            PipeMap::PIPE_LT => None, // -> J
            PipeMap::PIPE_LB => None, // -> 7
            _ => prev_pipe_char,
        };
        // println!("prev_pipe_char: {:?}", (prev_pipe_char, char));
    });

    chars_inside_of_the_loop.len()
}

pub fn run() -> usize {
    run_str(&help::read_data())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_run() {
        assert_eq!(run(), 529);
    }

    const LOOP: &str = r#"
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
    "#;

    #[test]
    fn test_run() {
        assert_eq!(run_str(LOOP), 4);
    }

    const LOOP_S: &str = r#"
...........
.S-------7.
.L------7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|oo|.
.L--J.L--J.
...........
    "#;

    #[test]
    fn test_run_s() {
        assert_eq!(run_str(LOOP_S), 2);
    }

    const LOOP_1: &str = r#"
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
    "#;

    #[test]
    fn test_run_1() {
        assert_eq!(run_str(LOOP_1), 8);
    }

    const LOOP_2: &str = r#"
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
    "#;

    #[test]
    fn test_run_2() {
        assert_eq!(run_str(LOOP_2), 10);
    }
}
