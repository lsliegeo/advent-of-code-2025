mod parse_input_file;

use parse_input_file::parse_file;

const DAY: u8 = 5;
const SAMPLE_INPUT: &'static str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

fn parse_input(raw_input: String) -> (Vec<(usize, usize)>, Vec<usize>) {
    let parts: Vec<&str> = raw_input.split("\n\n").collect();
    let fresh_id_ranges: Vec<(usize, usize)> = parts[0]
        .lines()
        .map(|line| {
            let parts: Vec<usize> = line.split("-").map(|part| part.parse().unwrap()).collect();
            (parts[0], parts[1])
        })
        .collect();
    let ids: Vec<usize> = parts[1].lines().map(|line| line.parse().unwrap()).collect();
    (fresh_id_ranges, ids)
}

fn main() {
    assert_eq!(part_one(parse_input(SAMPLE_INPUT.to_string())), 3);
    println!(
        "Solution for part one is: {}",
        part_one(parse_input(parse_file(DAY)))
    );
    assert_eq!(part_two(parse_input(SAMPLE_INPUT.to_string())), 14);
    println!(
        "Solution for part two is: {}",
        part_two(parse_input(parse_file(DAY)))
    );
}

fn part_one((fresh_id_ranges, ids): (Vec<(usize, usize)>, Vec<usize>)) -> usize {
    ids.iter()
        .filter(|&&id| {
            fresh_id_ranges
                .iter()
                .any(|&(start, end)| start <= id && id <= end)
        })
        .count()
}

fn part_two((mut fresh_id_ranges, _): (Vec<(usize, usize)>, Vec<usize>)) -> usize {
    // As long as ranges have overlap, merge them
    loop {
        let mut to_merge: Option<(usize, usize)> = None;
        'abc: for (i, range_1) in fresh_id_ranges.iter().enumerate() {
            for (j, range_2) in fresh_id_ranges.iter().enumerate().skip(i + 1) {
                if ranges_have_overlap(range_1, range_2) {
                    to_merge = Some((i, j));
                    break 'abc;
                }
            }
        }

        if let Some((i, j)) = to_merge {
            fresh_id_ranges[i] = merge_ranges(&fresh_id_ranges[i], &fresh_id_ranges[j]);
            fresh_id_ranges.remove(j);
        } else {
            break;
        }
    }

    // Count the remaining non-overlapping ranges
    fresh_id_ranges
        .iter()
        .map(|&(start, end)| end - start + 1)
        .sum()
}

fn ranges_have_overlap(range1: &(usize, usize), range2: &(usize, usize)) -> bool {
    range1.0 <= range2.1 && range2.0 <= range1.1
}

fn merge_ranges(range1: &(usize, usize), range2: &(usize, usize)) -> (usize, usize) {
    (
        std::cmp::min(range1.0, range2.0),
        std::cmp::max(range1.1, range2.1),
    )
}
