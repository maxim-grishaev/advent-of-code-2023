use std::collections::HashMap;

pub type ColorMap<'a> = HashMap<&'a str, u32>;

// only 12 red cubes, 13 green cubes, and 14 blue cubes
type MaxColors = [(&'static str, u32); 3];
const COLORS_CONSTRAINT: MaxColors = [("red", 12), ("green", 13), ("blue", 14)];
// ColorMap::from(MAX_COLORS);

pub fn is_valid_run<'a>(run: &ColorMap<'a>) -> bool {
    COLORS_CONSTRAINT
        .iter()
        .all(|(color, balls_max)| balls_max >= run.get(color).unwrap_or(&0))
}

pub struct GameResult<'a> {
    pub runs: Vec<ColorMap<'a>>,
    pub game_no: u32,
}

struct FileFormat {
    game_prefix: &'static str,
    game_to_run_sep: &'static str,
    color_to_ball_sep: &'static str,
    color_in_run_sep: &'static str,
    runs_sep: &'static str,
}
static CFG: FileFormat = FileFormat {
    game_prefix: "Game ",
    game_to_run_sep: ":",
    color_to_ball_sep: " ",
    color_in_run_sep: ",",
    runs_sep: ";",
};

fn parse_color_line<'a>(color_line: &'a str) -> (&'a str, u32) {
    let mut split = color_line.trim().split(CFG.color_to_ball_sep);
    let count: u32 = split.next().unwrap().parse().unwrap();
    let name: &'a str = split.next().unwrap();
    (name, count)
}

fn create_one_run<'a>(run_str: &'a str) -> ColorMap {
    run_str
        .split(CFG.color_in_run_sep)
        .map(parse_color_line)
        .collect()
}

fn read_game_no(game_line: &str) -> u32 {
    game_line
        .replace(CFG.game_prefix, "")
        .parse::<u32>()
        .unwrap()
}

pub fn parse_line<'a>(line: &'a str) -> GameResult {
    let mut split = line.split(CFG.game_to_run_sep);
    let game_str = split.next().unwrap();
    let all_runs_str = split.next().unwrap();
    return GameResult {
        game_no: read_game_no(game_str),
        runs: all_runs_str
            .split(CFG.runs_sep)
            .map(create_one_run)
            .collect(),
    };
}
