mod parse_input_file;

use parse_input_file::parse_file;

const DAY: u8 = 3;
const SAMPLE_INPUT: &'static str = "987654321111111
811111111111119
234234234234278
818181911112111";

fn main() {
    assert_eq!(
        max_joltage(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 2),
        98
    );
    assert_eq!(
        max_joltage(vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 2),
        89
    );
    assert_eq!(
        max_joltage(vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 2),
        78
    );
    assert_eq!(
        max_joltage(vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 2),
        92
    );
    assert_eq!(part_one(parse_input(SAMPLE_INPUT.to_string())), 357);
    println!(
        "Solution for part one is: {}",
        part_one(parse_input(parse_file(DAY)))
    );
    assert_eq!(
        max_joltage(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 12),
        987654321111
    );
    assert_eq!(
        max_joltage(vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 12),
        811111111119
    );
    assert_eq!(
        max_joltage(vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 12),
        434234234278
    );
    assert_eq!(
        max_joltage(vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 12),
        888911112111
    );
    assert_eq!(
        part_two(parse_input(SAMPLE_INPUT.to_string())),
        3121910778619
    );
    println!(
        "Solution for part two is: {}",
        part_two(parse_input(parse_file(DAY)))
    );
}

fn parse_input(raw_input: String) -> Vec<Vec<usize>> {
    raw_input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn part_one(input: Vec<Vec<usize>>) -> usize {
    let mut total_joltage: usize = 0;
    for batteries in input {
        total_joltage += max_joltage(batteries, 2);
    }
    total_joltage
}

fn max_with_index_stupid(vector: &[usize]) -> (usize, usize) {
    let mut max_index: usize = 0;
    let mut max_value: usize = 0;
    for (index, &value) in vector.iter().enumerate() {
        if value > max_value {
            max_index = index;
            max_value = value;
        }
    }
    (max_index, max_value)
}

fn max_joltage(batteries: Vec<usize>, number_batteries: usize) -> usize {
    let mut result: usize = 0;
    let mut index: usize = 0;
    for i in 0..number_batteries {
        let number_batteries_to_pick_afterwards = number_batteries - i - 1;
        let (max_index, max_value) = max_with_index_stupid(
            &batteries[index..batteries.len() - number_batteries_to_pick_afterwards],
        );
        index += max_index + 1;
        result = result * 10 + max_value;
    }
    result
}

fn part_two(input: Vec<Vec<usize>>) -> usize {
    let mut total_joltage: usize = 0;
    for batteries in input {
        total_joltage += max_joltage(batteries, 12);
    }
    total_joltage
}
