use crate::util;

#[derive(Debug)]
pub struct NumPos {
    line_no: usize,
    col_no: usize,
    pub num: u32,
    digits_count: usize,
    // raw: Vec<char>,
}

fn val(line_no: usize, col_no: usize, num: Vec<char>) -> NumPos {
    NumPos {
        line_no: line_no.clone(),
        col_no: col_no.clone(),
        num: num.iter().collect::<String>().parse::<u32>().unwrap(),
        digits_count: num.len(),
        // raw: num,
    }
}

pub fn get_char_at(schema: &Vec<&str>, line_no: &isize, col_no: &isize) -> Option<char> {
    if line_no < &0 || col_no < &0 || *line_no >= schema.len() as isize {
        return None;
    }
    return schema[*line_no as usize].chars().nth(*col_no as usize);
}

pub fn is_adjacent(num: &NumPos, gear_coords: &(usize, usize)) -> bool {
    let distance = (num.line_no as isize - gear_coords.0 as isize).abs();
    if distance > 1 {
        return false;
    }
    let adjacent_coords = get_adjacent_coords(num);
    adjacent_coords.iter().any(|(line_no, col_no)| {
        *line_no == gear_coords.0 as isize && *col_no == gear_coords.1 as isize
    })
}

pub fn get_adjacent_coords(np: &NumPos) -> Vec<(isize, isize)> {
    let col_left: isize = np.col_no as isize - np.digits_count as isize - 1;
    let col_right = np.col_no as isize;

    let line_top = np.line_no as isize - 1;
    let line_cur = (np.line_no) as isize;
    let line_btm = (np.line_no + 1) as isize;

    let mut adjacent_coords: Vec<(isize, isize)> = vec![];

    // left vertical
    adjacent_coords.push((line_top, col_left));
    adjacent_coords.push((line_cur, col_left));
    adjacent_coords.push((line_btm, col_left));

    // horisontal
    (0..np.digits_count).for_each(|i| {
        adjacent_coords.push((line_top, col_left + 1 + i as isize));
        adjacent_coords.push((line_btm, col_left + 1 + i as isize));
    });

    // right vertical
    adjacent_coords.push((line_top, col_right));
    adjacent_coords.push((line_cur, col_right));
    adjacent_coords.push((line_btm, col_right));

    adjacent_coords
}

fn is_symbol(c: char) -> bool {
    !c.is_numeric() && c != '.'
}

pub fn is_detail(schema: &Vec<&str>, np: &NumPos) -> bool {
    get_adjacent_coords(np).iter().any(|(line_no, col_no)| {
        match get_char_at(schema, line_no, col_no) {
            Some(c) => is_symbol(c),
            None => return false,
        }
    })
}

pub fn find_all_numbers(schema: &Vec<&str>) -> Vec<NumPos> {
    let mut nums: Vec<NumPos> = vec![];
    loop {
        let (line_no, col_no) = match nums.last() {
            Some(last) => (last.line_no, last.col_no),
            None => (0, 0),
        };
        match find_number(schema, line_no, col_no) {
            Some(found) => {
                nums.push(found);
                continue;
            }
            None => return nums,
        }
    }
}

pub fn find_number(
    schema: &Vec<&str>,
    line_no_start: usize,
    col_no_start: usize,
) -> Option<NumPos> {
    if line_no_start >= schema.len() {
        return None;
    }
    let mut line_no = line_no_start;
    let mut col_no = col_no_start;
    let mut num: Vec<char> = vec![];

    // println!("start_pos: {:?} {:?}", line_no, col_no);

    let mut cur_line = schema[line_no].chars();
    // println!("line: {:?}", cur_line);
    if col_no > 0 {
        cur_line.nth(col_no - 1);
    }

    // println!("schema: {:?}", schema);
    // println!("cur_line: {:?}", cur_line);
    loop {
        // println!("loop: {:?} {:?} {:?}", line_no, col_no, cur_line);
        match cur_line.next() {
            Some(c) => {
                if c.is_numeric() {
                    num.push(c);
                    col_no += 1;
                    continue;
                }
                // end of number
                if num.len() > 0 {
                    // println!("end of number. num: {:?} @ {:?}", num, line_no);
                    return Some(val(line_no, col_no, num));
                }
                col_no += 1;
            }
            // end of line
            None => {
                if num.len() > 0 {
                    // println!("end of line. num: {:?} @ {:?}", num, line_no);
                    return Some(val(line_no, col_no, num));
                }

                // goto next line
                line_no += 1;
                col_no = 0;

                // end of schema?
                if line_no >= schema.len() {
                    return None;
                }
                // println!("NEXT LINE!");
                cur_line = schema[line_no].chars();
                // println!("line: {:?}", cur_line);
            }
        }
    }
    // None
}

pub fn read_schema() -> String {
    util::fs::read_file(file!(), "engine.txt")
}

#[cfg(test)]
mod test {
    use super::*;
    use insta::assert_debug_snapshot;

    const TEST_INPUT: &str = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
    "#;

    #[test]
    fn find_all() {
        let schema: Vec<&str> = TEST_INPUT.trim().lines().collect();
        let nums = find_all_numbers(&schema);
        assert_debug_snapshot!(nums);
    }

    #[test]
    fn get_adjacent_coords_snapshot() {
        let schema: Vec<&str> = TEST_INPUT.trim().lines().collect();
        let npo = find_number(&schema, 0, 0);
        let np = npo.unwrap();
        println!("{:?}", np);
        assert_debug_snapshot!(get_adjacent_coords(&np));
    }
}
