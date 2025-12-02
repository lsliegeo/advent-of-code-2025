mod parse_input_file;
use parse_input_file::parse_file;

const SAMPLE_INPUT: &'static str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

fn main() {
    assert_eq!(part_one(parse_input(SAMPLE_INPUT.to_string())), 3);
    println!(
        "Solution for part one is: {}",
        part_one(parse_input(parse_file(1)))
    );
    assert_eq!(part_two(parse_input(SAMPLE_INPUT.to_string())), 6);
    assert_eq!(part_two(parse_input("R1000".to_string())), 10);
    assert_eq!(part_two(parse_input("L1000".to_string())), 10);
    assert_eq!(part_two(parse_input("R50\nL5".to_string())), 1);
    assert!(part_two(parse_input(parse_file(1))) > 2256);
    println!(
        "Solution for part two is: {}",
        part_two(parse_input(parse_file(1)))
    );
}

fn parse_input(raw_input: String) -> Vec<(char, usize)> {
    raw_input
        .lines()
        .map(|line| {
            (
                line.chars().next().expect("Can't parse direction"),
                line[1..].parse().expect("Can't parse amount"),
            )
        })
        .collect()
}

fn part_one(input: Vec<(char, usize)>) -> usize {
    let mut times_at_zero: usize = 0;
    let mut current_pointer: isize = 50;
    for (direction, amount) in input {
        if direction == 'L' {
            current_pointer -= amount as isize;
        } else {
            current_pointer += amount as isize;
        }
        current_pointer %= 100;
        if current_pointer == 0 {
            times_at_zero += 1;
        }
    }
    times_at_zero
}

fn part_two(input: Vec<(char, usize)>) -> usize {
    let mut times_at_zero: usize = 0;
    let mut current_pointer: isize = 50;
    for (direction, amount) in input {
        let new_pointer: isize;
        if direction == 'L' {
            new_pointer = current_pointer - amount as isize;
            if new_pointer <= 0 {
                if current_pointer != 0 {
                    times_at_zero += 1;
                }
                times_at_zero += ((amount as isize - current_pointer) / 100).unsigned_abs();
            }
        } else {
            new_pointer = current_pointer + amount as isize;
            if new_pointer >= 100 {
                times_at_zero += new_pointer as usize / 100;
            }
        }
        current_pointer = new_pointer.rem_euclid(100);
    }
    times_at_zero
}
