use crate::d03::help;

fn find_chars(schema: &Vec<&str>, c: char) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = vec![];
    for (line_no, line) in schema.iter().enumerate() {
        for (col_no, ch) in line.chars().enumerate() {
            if ch == c {
                res.push((line_no, col_no));
            }
        }
    }
    res
}

fn find_adjacent_nums(nums: &Vec<help::NumPos>, gear_coords: &(usize, usize)) -> Vec<u32> {
    let mut res: Vec<u32> = vec![];
    for num in nums {
        if help::is_adjacent(num, gear_coords) {
            res.push(num.num);
        }
    }
    res
}

pub fn run() -> u32 {
    let ngn = help::read_schema();
    let schema = ngn.lines().collect();
    let gears = find_chars(&schema, '*');
    let all_nums = help::find_all_numbers(&schema);
    gears
        .iter()
        .map(|gear_coords| {
            let nums = find_adjacent_nums(&all_nums, &gear_coords);
            // println!("gear_coords: {:?}", gear_coords);
            // println!("nums: {:?}", nums);
            if nums.len() < 2 {
                return 0;
            }
            nums.iter().copied().reduce(|a, b| a * b).unwrap()
        })
        .sum()
}
