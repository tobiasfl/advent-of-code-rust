use std::{fs, iter::successors};

const INFILE_PATH: &str = "../infiles/2019/day1.in";

fn main() {
    let puzzle_input = fs::read_to_string(INFILE_PATH)
        .expect("Did not find {INFILE_PATH}");
    
    let parsed_puzzle_input = parse_input(puzzle_input);

    let sum_of_fuel = sum_of_fuel_requirements_naive(&parsed_puzzle_input);
    println!("sum_of_fuel {sum_of_fuel}");

    let sum_of_fuel_with_fuel_mass = sum_of_fuel_requirements_with_fuel_mass(&parsed_puzzle_input);
    println!("sum_of_fuel {sum_of_fuel_with_fuel_mass}");
}

fn parse_input(puzzle_input: String) -> Vec<f64> {
    puzzle_input.split_whitespace()
        .map(|x| x.parse().expect("input file contained an invalid number"))
        .collect()
}

fn mass_to_fuel_requirement(mass: f64) -> f64 {
    (mass / 3.0).floor() - 2.0
}

fn sum_of_fuel_requirements_naive(input: &Vec<f64>) -> f64 {
    input.iter()
        .map(|x| mass_to_fuel_requirement(*x))
        .sum::<f64>()
}

fn sum_of_fuel_requirements_with_fuel_mass(input: &Vec<f64>) -> f64 {
        input.iter()
            .map(|inp|
                successors(Some(*inp), |curr| Some(mass_to_fuel_requirement(*curr)))
                    .take_while(|mass| *mass > 0.0)
                    .skip(1)
                    .sum::<f64>())
            .sum()
}
