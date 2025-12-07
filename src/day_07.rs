mod parse_input_file;

use parse_input_file::parse_file;
use std::collections::{HashMap, HashSet};

const DAY: u8 = 7;
const SAMPLE_INPUT: &'static str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

fn main() {
    assert_eq!(part_one(parse_input(SAMPLE_INPUT.to_string())), 21);
    println!(
        "Solution for part one is: {}",
        part_one(parse_input(parse_file(DAY)))
    );
    assert_eq!(part_two(parse_input(SAMPLE_INPUT.to_string())), 40);
    println!(
        "Solution for part two is: {}",
        part_two(parse_input(parse_file(DAY)))
    );
}

fn parse_input(raw_input: String) -> (usize, Vec<Vec<usize>>) {
    let start_pos: usize = raw_input
        .lines()
        .next()
        .unwrap()
        .chars()
        .position(|c| c == 'S')
        .unwrap();
    let splitters: Vec<Vec<usize>> = raw_input
        .lines()
        .skip(1)
        .map(|line| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c == '^')
                .map(|(i, _)| i)
                .collect()
        })
        .collect();
    (start_pos, splitters)
}

fn part_one((start_pos, splitters): (usize, Vec<Vec<usize>>)) -> usize {
    let mut number_splits: usize = 0;
    let mut current_indices: HashSet<usize> = HashSet::new();
    current_indices.insert(start_pos);
    for indices_to_split in splitters {
        let mut new_indices: HashSet<usize> = current_indices.clone();
        for index_to_split in indices_to_split {
            if current_indices.contains(&index_to_split) {
                new_indices.remove(&index_to_split);
                new_indices.insert(index_to_split + 1);
                new_indices.insert(index_to_split - 1);
                number_splits += 1;
            }
        }
        current_indices = new_indices
    }
    number_splits
}

fn part_two((start_pos, splitters): (usize, Vec<Vec<usize>>)) -> usize {
    branch_and_bound(start_pos, 1, &splitters, &mut HashMap::new())
}

fn branch_and_bound(
    current_pos: usize,
    current_row: usize,
    splitters: &Vec<Vec<usize>>,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if !cache.contains_key(&(current_pos, current_row)) {
        // Cache miss, calculate and put in cache
        let result: usize;
        if current_row >= splitters.len() {
            // Reached the end
            result = 1;
        } else if splitters[current_row].contains(&current_pos) {
            // Split
            result = branch_and_bound(current_pos + 1, current_row + 1, splitters, cache)
                + branch_and_bound(current_pos - 1, current_row + 1, splitters, cache);
        } else {
            // Continue downwards
            result = branch_and_bound(current_pos, current_row + 1, &splitters, cache);
        }
        cache.insert((current_pos, current_row), result);
    }
    cache[&(current_pos, current_row)]
}
