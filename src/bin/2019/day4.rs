use std::slice;
use std::collections::HashMap;

const PUZZLE_INPUT: &str = "197487-673251";

fn main() {
    let (min, max) = parse_input(PUZZLE_INPUT).unwrap();

    let valid_passwords: Vec<String> = (min..=max)
        .map(|x| x.to_string())
        .filter(|x| x.len() == 6)
        .filter(|x| is_sorted(x.chars().flat_map(|c| c.to_digit(10)).collect()))
        .filter(|x| contains_at_least_n_equal_in_a_row(2, x))
        .collect();
    println!("{:?}", valid_passwords.len());

    let valid_passwords: Vec<String> = (min..=max)
        .map(|x| x.to_string())
        .filter(|x| x.len() == 6)
        .filter(|x| is_sorted(x.chars().flat_map(|c| c.to_digit(10)).collect()))
        .filter(|x| contains_exact_n_equal_in_a_row(2, x))
        .collect();
    println!("{:?}", valid_passwords.len());
}

fn parse_input(puzzle_input: &str) -> Option<(u32, u32)> {
    let splitted: Vec<&str> = puzzle_input.split('-').collect();
    let fst = splitted.get(0)?;
    let snd = splitted.get(1)?;

    Some((fst.parse().ok()?,  snd.parse().ok()?))
}

fn is_sorted(nums: Vec<u32>) -> bool {
    let mut prev = 0;
    for num in nums {
        if num < prev {
            return false
        }
        prev = num
    }
    true
}

fn contains_at_least_n_equal_in_a_row(n: usize, sorted_s: &str) -> bool {
    get_countings(sorted_s).values().filter(|count| **count >= n).count() > 0
}

fn contains_exact_n_equal_in_a_row(n: usize, sorted_s: &str) -> bool {
    get_countings(sorted_s).values().filter(|count| **count == n).count() > 0
}

fn get_countings(s: &str) -> HashMap<&u8, usize> {
    let mut countings = HashMap::new();
    for k in s.as_bytes() {
        if countings.contains_key(k) {
            countings.get_mut(k).map(|count| *count += 1);
        } else {
            countings.insert(k, 1);
        }
    }
    countings
}
