use std::fs;

pub fn parse_file(day: u8) -> String {
    let file_name = format!("input/{:02}.txt", day);
    fs::read_to_string(file_name)
        .unwrap_or_else(|_| panic!("Can't open input file for file {}", day))
}
