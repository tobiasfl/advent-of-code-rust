use std::fs;

const INFILE_PATH: &str = "../infiles/2023/day9.in";

fn main() {
    let puzzle_input = fs::read_to_string(INFILE_PATH)
        .expect("Did not find {INFILE_PATH}");
    
    let parsed = parse_puzze_input(&puzzle_input);

    let sum_of_last_ns: i64 = parsed.iter().map(|nums| find_next_n(nums)).sum();
    println!("{sum_of_last_ns}");

    let reversed: Vec<_> = parsed.iter().map(|nums| nums.clone().into_iter().rev().collect()).collect();
    let sum_of_first_ns: i64 = reversed.iter().map(|nums| find_next_n(nums)).sum();
    println!("{sum_of_first_ns}");
}

fn parse_puzze_input(input: &str) -> Vec<Vec<i64>> {
    input.split('\n')
        .map(|line| line.split_whitespace().flat_map(|n| n.parse()).collect())
        .filter(|nums: &Vec<_>| !nums.is_empty())
        .collect()
}

fn find_next_n(nums: &Vec<i64>) -> i64 {
    let mut all_diffs = vec![nums.clone()];
    let mut curr_nums = nums.clone();
    while curr_nums.iter().any(|n| *n != 0) {
        curr_nums = diffs(&curr_nums);
        all_diffs.push(curr_nums.clone());
    }

    extrapolate(&all_diffs)
}

fn extrapolate(all_diffs: &Vec<Vec<i64>>) -> i64 {
    
    let mut num = 0;
    for curr_diffs in all_diffs.iter().rev() {
        if let Some(diff) = curr_diffs.last() {
            num += diff;
        }
    }
    num
}

fn diffs(nums: &Vec<i64>) -> Vec<i64> {
     (0..nums.len()).zip(1..)
         .flat_map(|(x, y)| 
              if let Some([curr, next]) = nums.get(x..=y) {
                Some(next - curr)
             } else {
                None
             }
         )
         .collect()
}
