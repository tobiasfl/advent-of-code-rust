use std::fs;

const INFILE_PATH: &str = "../infiles/2023/day6.in";

fn main() {
    let puzzle_input = fs::read_to_string(INFILE_PATH)
        .expect("Did not find {INFILE_PATH}");

    let parsed_input = parse_input1(&puzzle_input);
    println!("{:?}", parsed_input);

    let multiplied_num_of_wins = parsed_input
        .map(|stats| possible_wins(stats).len())
        .into_iter()
        .reduce(|acc, e| acc * e);
    println!("{:?}", multiplied_num_of_wins);

    let input_2: (u64, u64) = (57726992,  291117211762026);
    println!("{:?}", possible_wins(input_2).len());
}

fn parse_input1(input: &str) -> [(u64, u64); 4] {
    let lines: Vec<Vec<u64>> = input.split('\n')
        .map(|line| line.split_whitespace().skip(1).map(|num| num.trim().parse().unwrap()).collect())
        .collect();
    if let Some([times, distances]) = lines.get(0..2) {
        return [(times[0], distances[0]), (times[1], distances[1]), (times[2], distances[2]), (times[3], distances[3])]
    }
    [(0, 0),(0, 0),(0, 0),(0, 0),]
}


// Brute force ftw
fn possible_wins((time, record_distance): (u64, u64)) -> Vec<u64> {
    (0..=time).map(|hold_time| hold_time * (time - hold_time))
        .filter(|dist_travelled| *dist_travelled > record_distance)
        .collect()
}
