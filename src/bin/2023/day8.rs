use std::{fs, collections::HashMap};
use itertools::Itertools;

const INFILE_PATH: &str = "../infiles/2023/day8.in";

fn main() {
    let puzzle_input = fs::read_to_string(INFILE_PATH)
        .expect("Did not find {INFILE_PATH}");
    
    let (instructions, map) = parse_puzzle_input(&puzzle_input);
    println!("{:?}", steps_to_ZZZ(instructions, &map));
    
    println!("{:?}", steps_to_z_all(instructions, &map));

}

fn parse_puzzle_input(input: &str) -> (&str, HashMap<String, (String, String)>) {
    if let Some([instructions, map]) = input.split("\n\n").collect::<Vec<&str>>().get(0..=1) {
        let mut parsed_map = HashMap::new();
        for line in map.split('\n') {
            if let Some([a, _, l, r]) = line.split_whitespace().collect::<Vec<&str>>().get(0..4) {
                let l = l.chars().filter(|c| c.is_alphabetic()).collect();
                let r = r.chars().filter(|c| c.is_alphabetic()).collect();
                parsed_map.insert(a.to_string(), (l, r));
            }
        }

        return (instructions, parsed_map)
    }
    ("", HashMap::new())
}

fn steps_to_ZZZ(instructions: &str, map: &HashMap<String, (String, String)>) -> u64 {
    let mut curr = "AAA";
    let mut steps = 0;
    for dir in instructions.chars().into_iter().cycle() {
        if curr == "ZZZ" {
            return steps; 
        }
        if let Some((l, r)) = map.get(curr) {
            if dir == 'L' {
                curr = l;
            } else if dir == 'R' {
                curr = r;
            }
        }
        steps += 1;
    }
    steps
}

fn steps_to_first_z(instructions: &str, map: &HashMap<String, (String, String)>, from: &str) -> u64 {
    let mut curr = from;
    let mut steps = 0;
    for dir in instructions.chars().into_iter().cycle() {
        if curr.ends_with('Z') {
            return steps; 
        }
        if let Some((l, r)) = map.get(curr) {
            if dir == 'L' {
                curr = l;
            } else if dir == 'R' {
                curr = r;
            }
        }
        steps += 1;
    }
    steps
}

fn steps_to_z_all(instructions: &str, map: &HashMap<String, (String, String)>) -> u64 {
    map.keys().into_iter()
        .filter(|k| k.ends_with('A'))
        .map(|s| steps_to_first_z(instructions, map, s))
        .reduce(|acc, e| lcm(acc, e))
        .unwrap()
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    (a * b) / gcd(a, b)
}
