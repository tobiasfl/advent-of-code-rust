use std::fs;
use std::slice;
use std::cmp;

const INFILE_PATH: &str = "../infiles/2023/day1.in";

fn main() {
    let puzzle_input = fs::read_to_string(INFILE_PATH)
        .expect("Did not find {INFILE_PATH}");
    
    let nums = extract_nums(puzzle_input.clone());
    let sum_of_nums = nums.into_iter().reduce(|acc, e| acc + e);
    println!("{:?}", sum_of_nums);
    
    let nums = extract_nums_2(puzzle_input);
    let sum_of_nums = nums.into_iter().reduce(|acc, e| acc + e);
    println!("{:?}", sum_of_nums);
}

fn extract_nums(puzzle_input: String) -> Vec<u32> {
    let mut result = Vec::new();
    for line in puzzle_input.split_whitespace() {
        let left = line.chars()
            .find(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10))
            .expect("no left digit");
        let right = line.chars()
            .rfind(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10))
            .expect("no right digit");
        result.push((left.unwrap() * 10) + right.unwrap());
    }
    result
}

fn extract_nums_2(puzzle_input: String) -> Vec<u32> {
    let mut result = Vec::new();
    for line in puzzle_input.split_whitespace() {
        let left = find_left_num(line);
        
        let right = find_right_num(line);

        result.push((left * 10) + right);
    }
    result
}

fn find_left_num(line: &str) -> u32 {
    let line_as_vec: Vec<char> = line.chars().collect();
    for i in 0..line_as_vec.len() {
        for window_len in [1, 3, 4, 5] {
            let window = &line_as_vec[i..cmp::min(i+window_len, line_as_vec.len())];
            if let Some(d) = to_digit(window) {
                return d;
            }
        }
    }
    0
}

fn find_right_num(line: &str) -> u32 {
    let line_as_vec: Vec<char> = line.chars().collect();
    for i in (0..=line_as_vec.len()).rev() {
        for window_len in [1, 3, 4, 5] {
            let slice_start_index = i-cmp::min(window_len, i);
            let window = &line_as_vec[slice_start_index..i];
            if let Some(d) = to_digit(window) {
                return d;
            }
        }
    }
    0
}

fn to_digit(s: &[char]) -> Option<u32> {
    match s {
        ['z', 'e', 'r', 'o'] => Some(0),
        ['o', 'n', 'e'] => Some(1),
        ['t', 'w', 'o'] => Some(2),
        ['t', 'h', 'r', 'e', 'e'] => Some(3),
        ['f', 'o', 'u', 'r'] => Some(4),
        ['f', 'i', 'v', 'e'] => Some(5),
        ['s', 'i', 'x'] => Some(6),
        ['s', 'e', 'v', 'e', 'n'] => Some(7),
        ['e', 'i', 'g', 'h', 't'] => Some(8),
        ['n', 'i', 'n', 'e'] => Some(9),
        [c] => c.to_digit(10),
        _ => None
    }
}
