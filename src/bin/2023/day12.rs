use std::collections::HashMap;
use std::fs;
use std::iter;
use itertools::Itertools;

const INFILE_PATH: &str = "../infiles/2023/day12.in";

fn main() {
    let puzzle_input = fs::read_to_string(INFILE_PATH)
        .expect("Did not find {INFILE_PATH}");
    
    let lines = parse_puzzle_input(&puzzle_input);
    let n = possible_arrangements(&lines);
    println!("n: {n}");
    let unfolded_lines: Vec<_> = lines.iter().map(&unfold).collect();
    let unfolded_n = possible_arrangements(&unfolded_lines);
    println!("unfolded_n: {unfolded_n}");
}

fn parse_puzzle_input(input: &str) -> Vec<(String, Vec<u64>)> {
    let mut result = Vec::new();

    for line in input.split_terminator('\n') {
        if let Some((chars, nums)) = line.split_once(' ') {
            let parsed_nums = nums.split(',').flat_map(|n| n.parse()).collect();
            result.push((chars.to_string(), parsed_nums));
        }
    }
    
    result
}

fn possible_arrangements(all_rows: &Vec<(String, Vec<u64>)>) -> u64 {
    let mut memoize_table = HashMap::new();
    all_rows.iter().map(|r| {find_arrangements(0, 0, 0, r, &mut memoize_table)}).sum()
}

fn unfold<'a>((row, nums): &'a (String, Vec<u64>)) -> (String, Vec<u64>) {
    let new_row = iter::repeat(row).take(5).join("?");
    let new_nums = iter::repeat(nums.clone()).take(5).flatten().collect();

    (new_row, new_nums)
}

fn find_arrangements<'a>(char_i: usize, 
                     num_i: usize, 
                     group_size: u64, 
                     r@(row, nums): &'a (String, Vec<u64>), 
                     memoized_results: &mut HashMap<(usize, usize, u64, &'a (String, Vec<u64>)), u64>) -> u64 {
    // Out of groups
    if num_i == nums.len() - 1 
        && nums.get(num_i).is_some_and(|n| {*n == group_size}) {
        if row.get(char_i..).is_some_and(|rest| !rest.contains('#'))  {
            return 1
        } else {
            return 0
        }
    }
    
    if let Some(c) = row.chars().nth(char_i) {
        // At the end of a group
        if nums.get(num_i).is_some_and(|n| {*n == group_size}) {
            if c == '#' {
                return 0
            } else {
                return find_arrangements_memoized(char_i+1, num_i+1, 0,  r, memoized_results);
            }
        }

        // Just starting on a group
        if 0 == group_size {
            if c == '?' {
                // try both with and without assuming the ? is #
                return  find_arrangements_memoized(char_i+1, num_i, 1, r, memoized_results) 
                    + find_arrangements_memoized(char_i+1, num_i, 0, r, memoized_results)
            }
            if c == '.' {
                return  find_arrangements_memoized(char_i+1, num_i, 0, r, memoized_results)
            }
            if c == '#' {
                return  find_arrangements_memoized(char_i+1, num_i, 1, r, memoized_results) 
            }
        }

        // Assume in the middle of a group
        if c == '.' {
            return 0
        } else {
            return find_arrangements_memoized(char_i+1, num_i, group_size+1, r, memoized_results) 
        }
    }

    0
}

fn find_arrangements_memoized<'a>(char_i: usize, 
                     num_i: usize, 
                     group_size: u64, 
                     r: &'a (String, Vec<u64>), 
                     memoized_results: &mut HashMap<(usize, usize, u64, &'a (String, Vec<u64>)), u64>) -> u64 {
    if let Some(result) = memoized_results.get(&(char_i, num_i, group_size, r)) {
        return *result
    }
    let result = find_arrangements(char_i, num_i, group_size, r, memoized_results);
    memoized_results.insert((char_i, num_i, group_size, r), result);
    result
}
