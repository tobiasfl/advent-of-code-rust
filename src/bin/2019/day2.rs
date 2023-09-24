use std::fs;

const INFILE_PATH: &str = "infiles/2019/day2.in";

//Opcodes: 1,2,99
//1: followed by three positions, add the numbers on the two first and put them in the third
//2: followed by three positions, multiply the numbers on the two first and put them in the third
//99: halt
//Once we are done with a position, move forward 4 positions
//Before running the program, replace position 1 with the value 12 and replace position 2 with the value 2. What value is left at position 0 after the program halts?

fn main() {
    let puzzle_input = fs::read_to_string(INFILE_PATH)
        .expect("Did not find {INFILE_PATH}");

    let program = parse_input(&puzzle_input);
    println!("First: {:?}", run_and_get_program_output(&program, 12, 2));

    for noun in 0..100 {
        for verb in 0..100 {
            if run_and_get_program_output(&program, noun, verb) == 19690720 {
                println!("Second: {}", 100 * noun + verb);
            }
        }
    }
}

fn parse_input(puzzle_input: &String) -> Vec<usize> {
    puzzle_input.split(',')
        .map(|x| match x.parse() { 
            Ok(parsed_x) => parsed_x,
            Err(_) => 0})
        .collect()
}

fn run_and_get_program_output(program: &Vec<usize>, noun: usize, verb: usize) -> usize {
    let mut program_copy = program.to_owned();
    prepare_program(&mut program_copy, noun, verb);
    run_program(&mut program_copy);
    program_copy[0]
}

fn prepare_program(program: &mut Vec<usize>, noun: usize, verb: usize) {
    program[1] = noun;
    program[2] = verb;
}

fn run_program(program: &mut Vec<usize>) {
    let mut i = Some(0);

    while i.is_some() {
        i = run_instr_at_index(i.expect("i was invalid"), program);
    }
}

fn run_instr_at_index(index: usize, program: &mut Vec<usize>) -> Option<usize> {
    match program.get(index..index+4) {
        Some(&[opcode, x_index, y_index, output_index]) => {
            let x_val = program.get(x_index).map(|x_val_ref| *x_val_ref).unwrap_or_default();
            let y_val = program.get(y_index).map(|y_val_ref| *y_val_ref).unwrap_or_default();
            match opcode {
                1 => {
                    match program.get_mut(output_index) {
                        Some(output_ref) => {
                            *output_ref = x_val + y_val;
                            Some(index+4)
                        },
                        _ => None
                    }
                },
                2 => {
                    match program.get_mut(output_index) {
                        Some(output_ref) => {
                            *output_ref = x_val * y_val;
                            Some(index+4)
                        },
                        _ => None
                    }
                },
                99 => {
                    None
                },
                _ => None
            }
        },
        _ => None
    }
}
