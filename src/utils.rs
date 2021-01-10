use std::fs;

pub fn get_input(day: u8) -> String {
    let input_path = format!("{}/inputs/day{:0>2}.txt", env!("CARGO_MANIFEST_DIR"), day);
    fs::read_to_string(input_path).unwrap()
}

#[allow(dead_code)]
pub fn get_sample(day: u8) -> String {
    let input_path = format!("{}/sample/day{:0>2}.txt", env!("CARGO_MANIFEST_DIR"), day);
    fs::read_to_string(input_path).unwrap()
}
