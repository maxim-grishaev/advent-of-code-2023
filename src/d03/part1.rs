use crate::d03::help;

pub fn run() -> u32 {
    let ngn = help::read_schema();
    let schema = ngn.lines().collect();
    let all_nums = help::find_all_numbers(&schema);
    let num_pos: Vec<&help::NumPos> = all_nums
        .iter()
        .filter(|np| help::is_detail(&schema, np))
        .collect();
    // println!("part1: {:?}", help::get_adjacent_coords(num_pos[20]));
    num_pos.iter().map(|np| np.num).sum::<u32>()
}
