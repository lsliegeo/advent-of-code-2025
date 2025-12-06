mod parse_input_file;

use parse_input_file::parse_file;

const DAY: u8 = 6;
const SAMPLE_INPUT: &'static str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

fn main() {
    assert_eq!(solve(parse_part_one(SAMPLE_INPUT.to_string())), 4277556);
    println!(
        "Solution for part one is: {}",
        solve(parse_part_one(parse_file(DAY)))
    );
    assert_eq!(solve(parse_part_two(SAMPLE_INPUT.to_string())), 3263827);
    println!(
        "Solution for part two is: {}",
        solve(parse_part_two(parse_file(DAY)))
    );
}

fn parse_part_one(raw_input: String) -> Vec<(Vec<usize>, char)> {
    let mut parsed_input = Vec::new();
    let index_split = raw_input.find(|c| c == '*' || c == '+').unwrap();
    let operations: Vec<char> = raw_input[index_split..]
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    let all_numbers: Vec<usize> = raw_input[..index_split]
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    for i in 0..operations.len() {
        let numbers: Vec<usize> = all_numbers
            .iter()
            .enumerate()
            .filter(|(index, _)| index % operations.len() == i)
            .map(|(_, &value)| value)
            .collect();
        parsed_input.push((numbers, operations[i]))
    }
    parsed_input
}

fn parse_part_two(raw_input: String) -> Vec<(Vec<usize>, char)> {
    let mut parsed_input: Vec<(Vec<usize>, char)> = Vec::new();
    let mut lines: Vec<String> = raw_input.lines().map(|line| line.to_string()).collect();
    let max_line_length = lines.iter().map(|line| line.len()).max().unwrap();
    lines = lines
        .into_iter()
        .map(|line| format!("{:<max_line_length$}", line))
        .collect();
    let mut operation: Option<char> = None;
    let mut numbers: Vec<usize> = Vec::new();
    for i in 0..max_line_length + 1 {
        if operation.is_none() {
            // new problem, store the operation
            operation = Some(lines.iter().last().unwrap().chars().nth(i).unwrap());
        }
        if i == max_line_length
            || lines
                .iter()
                .map(|line| line.chars().nth(i).unwrap())
                .all(|c| c.is_whitespace())
        {
            // all empty, problem is parsed
            parsed_input.push((numbers, operation.unwrap()));
            numbers = Vec::new();
            operation = None;
        } else {
            // still parsing numbers
            numbers.push(
                lines
                    .iter()
                    .map(|line| line.chars().nth(i).unwrap())
                    .filter(|c| c.is_digit(10))
                    .collect::<String>()
                    .parse()
                    .unwrap(),
            )
        }
    }
    parsed_input
}

/*
123 328  51 64
 45 64  387 23
  6 98  215 314

1 24 356
369 248 8
32 581 175
623 431 4
*/

fn solve(input: Vec<(Vec<usize>, char)>) -> usize {
    let mut total_result: usize = 0;
    for (numbers, operation) in input {
        let mut numbers_iter = numbers.iter();
        let mut result = *numbers_iter.next().unwrap();
        for number in numbers_iter {
            match operation {
                '+' => result += number,
                '*' => result *= number,
                _ => panic!("Invalid operation"),
            }
        }
        total_result += result;
    }
    total_result
}
