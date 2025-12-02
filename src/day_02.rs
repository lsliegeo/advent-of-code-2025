mod parse_input_file;
use parse_input_file::parse_file;

const DAY: u8 = 2;
const SAMPLE_INPUT: &'static str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

fn main() {
    assert_eq!(part_one(parse_input(SAMPLE_INPUT.to_string())), 1227775554);
    println!(
        "Solution for part one is: {}",
        part_one(parse_input(parse_file(DAY)))
    );
    assert_eq!(is_invalid_id_part_two(111), true);
    assert_eq!(part_two(parse_input(SAMPLE_INPUT.to_string())), 4174379265);
    println!(
        "Solution for part two is: {}",
        part_two(parse_input(parse_file(DAY)))
    );
}

fn parse_input(raw_input: String) -> Vec<(usize, usize)> {
    raw_input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|line| {
            let vector: Vec<usize> = line
                .split("-")
                .map(|number| number.parse().unwrap())
                .collect();
            (vector[0], vector[1])
        })
        .collect()
}

fn part_one(input: Vec<(usize, usize)>) -> usize {
    let mut result: usize = 0;
    for range in input {
        for id in range.0..range.1 + 1 {
            if is_invalid_id_part_one(id) {
                result += id;
            }
        }
    }
    result
}

fn is_invalid_id_part_one(id: usize) -> bool {
    if id.ilog10() % 2 == 0 {
        return false;
    }
    let id_str: String = id.to_string();
    let half_index: usize = id_str.len() / 2;
    id_str[..half_index] == id_str[half_index..]
}

fn part_two(input: Vec<(usize, usize)>) -> usize {
    let mut result: usize = 0;
    for range in input {
        for id in range.0..range.1 + 1 {
            if is_invalid_id_part_two(id) {
                result += id;
            }
        }
    }
    result
}

fn is_invalid_id_part_two(id: usize) -> bool {
    let string_length: usize = id.ilog10() as usize + 1;
    let id_str: String = id.to_string();
    'outer_loop: for length in 1..string_length / 2 + 1 {
        let number_chunks: usize = string_length / length;
        if number_chunks * length == string_length {
            for chunk_index in 1..number_chunks {
                if id_str[chunk_index * length..(chunk_index + 1) * length] != id_str[..length] {
                    continue 'outer_loop;
                }
            }
            return true
        }
    }
    false
}
