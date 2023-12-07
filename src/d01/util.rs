pub fn take_line_digits(line: &str) -> u32 {
    let mut l: u32 = 10;
    let mut r: u32 = 10;
    // iterate over the line's chars
    for (_i, c) in line.chars().enumerate() {
        let num = match c.to_digit(10) {
            None => continue,
            Some(n) => n,
        };

        if l == 10 {
            l = num;
            r = num;
        } else {
            r = num;
        }
    }
    return (l * 10 + r) as u32;
}
