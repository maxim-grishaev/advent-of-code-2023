use super::util;
use crate::help::fs;

const DGTS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn replace_digits(line: &str) -> String {
    let mut c = 0;
    DGTS.iter().fold(line.to_string(), |acc, x| {
        c += 1;
        acc.replace(x, [x, c.to_string().as_str(), x].concat().as_str())
    })
}

pub fn run() -> u32 {
    let input = fs::read_file(file!(), "input.txt");
    let sum = input
        .lines()
        .map(|line| {
            let ref new_line = replace_digits(line);
            let num = util::take_line_digits(new_line);
            // println!("{}", num);
            // println!("{} => {} => {}", line, new_line, num);
            return num;
        })
        .sum();

    return sum;
}
