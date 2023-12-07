use super::util;
use crate::help::fs;

pub fn run() -> u32 {
    let input = fs::read_file(file!(), "input.txt");
    let sum = input
        .lines()
        .map(|line| {
            let num = util::take_line_digits(line);
            // println!("{}", num);
            // println!("{} => {} => {}", line, new_line, num);
            return num;
        })
        .sum();

    return sum;
}
