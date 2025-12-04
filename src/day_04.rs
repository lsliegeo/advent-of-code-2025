mod parse_input_file;

use parse_input_file::parse_file;

const DAY: u8 = 4;
const SAMPLE_INPUT: &'static str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

fn main() {
    assert_eq!(part_one(parse_input(SAMPLE_INPUT.to_string())), 13);
    println!(
        "Solution for part one is: {}",
        part_one(parse_input(parse_file(DAY)))
    );
    assert_eq!(part_two(parse_input(SAMPLE_INPUT.to_string())), 43);
    println!(
        "Solution for part two is: {}",
        part_two(parse_input(parse_file(DAY)))
    );
}

fn parse_input(raw_input: String) -> Vec<Vec<bool>> {
    raw_input
        .lines()
        .map(|line| line.chars().map(|char| char == '@').collect())
        .collect()
}

fn part_one(input: Vec<Vec<bool>>) -> usize {
    let mut result: usize = 0;
    for (i, row) in input.iter().enumerate() {
        for (j, is_paper_roll) in row.iter().enumerate() {
            if *is_paper_roll {
                let mut paper_roll_neighbours: usize = 0;
                for (ii, jj) in neighbours(i, j, input.len(), row.len()) {
                    if input[ii][jj] {
                        paper_roll_neighbours += 1;
                    }
                }
                if paper_roll_neighbours < 4 {
                    result += 1;
                }
            }
        }
    }
    result
}

fn neighbour_offsets() -> [(isize, isize); 8] {
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
}

fn neighbours(i: usize, j: usize, max_i: usize, max_j: usize) -> Vec<(usize, usize)> {
    neighbour_offsets()
        .iter()
        .map(|(offset_i, offset_j)| (i as isize + offset_i, j as isize + offset_j))
        .filter(|&(i, j)| i >= 0 && j >= 0 && i < max_i as isize && j < max_j as isize)
        .map(|(i, j)| (i as usize, j as usize))
        .collect()
}

fn part_two(mut input: Vec<Vec<bool>>) -> usize {
    let mut total_removed: usize = 0;
    loop {
        let mut now_removed: usize = 0;

        for i in 0..input.len() {
            for j in 0..input[i].len() {
                if input[i][j] {
                    let mut paper_roll_neighbours: usize = 0;
                    for (ii, jj) in neighbours(i, j, input.len(), input[i].len()) {
                        if input[ii][jj] {
                            paper_roll_neighbours += 1;
                        }
                    }
                    if paper_roll_neighbours < 4 {
                        total_removed += 1;
                        now_removed += 1;
                        input[i][j] = false;
                    }
                }
            }
        }

        if now_removed == 0 {
            break;
        }
    }
    total_removed
}
