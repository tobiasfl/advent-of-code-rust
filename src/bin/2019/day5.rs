use std::fs;

const INFILE_PATH: &str = "infiles/2019/day5.in";

fn main() {
    let puzzle_input = fs::read_to_string(INFILE_PATH)
        .expect("Did not find {INFILE_PATH}");

    let program = parse_input(&puzzle_input);
}

fn parse_input(puzzle_input: &str) -> Vec<i32> {
    vec!()
}
