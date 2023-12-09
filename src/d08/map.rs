use std::collections::HashMap;

#[derive(Debug)]
pub struct Map {
    pub dir: String,
    pub nodes: HashMap<String, (String, String)>,
}

impl Map {
    pub const START_NODE: &str = "AAA";
    pub const END_NODE: &str = "ZZZ";

    pub const START_SUFFIX: &str = "A";
    pub const END_SUFFIX: &str = "Z";

    pub const LEFT: char = 'L';
    pub const RIGHT: char = 'R';
}

impl From<&str> for Map {
    fn from(str: &str) -> Self {
        let string = str.to_string();
        let mut lines = string.lines();

        let dir = lines.next().unwrap().trim().to_string();
        lines.next();

        let map = lines
            .map(|line| {
                let mut parts = line.split("=");
                let key = parts.next().unwrap().trim();
                let val = parts.next().unwrap().trim();

                let mut parts = val.split(",");
                let left = parts.next().unwrap().trim().replace("(", "");
                let right = parts.next().unwrap().trim().replace(")", "");

                (key.to_string(), (left.to_string(), right.to_string()))
            })
            .collect();

        Map { dir, nodes: map }
    }
}
