use std::path::Path;

pub fn read_file(cur_file: &str, sibling_path: &str) -> String {
    let cur_dir = Path::new(cur_file).parent().unwrap();
    let ref input_path = cur_dir.join(sibling_path);
    // println!("file: {}", input_path.clone().to_str().unwrap());
    return std::fs::read_to_string(input_path)
        .expect("Something went wrong reading the file {cur_file}/{sibling_path}");
}
