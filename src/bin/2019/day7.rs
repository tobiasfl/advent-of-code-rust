use std::collections::VecDeque;
use std::fs;
use itertools::Itertools;
use std::iter;

const INFILE_PATH: &str = "infiles/2019/day7.in";

//Opcodes: 1,2,99
//1: followed by three positions, add the numbers on the two first and put them in the third
//2: followed by three positions, multiply the numbers on the two first and put them in the third
//3: takes a single integer as input and saves it to the position given by its only parameter. For example, the instruction 3,50 would take an input value and store it at address 50. (THIS ONE SHOULD ALWAYS GET 1 AS INPUT)
//4: outputs the value of its only parameter. For example, the instruction 4,50 would output the value at address 50. (THIS ONE SHOULD PRINT WHATEVER VALUE IS AT THE GIVEN ADDRESS IF IT IS NON-ZERO THERE IS SOMETHING WRONG)
//99: halt
//Once we are done with a position, move forward 4 positions
//Before running the program, replace position 1 with the value 12 and replace position 2 with the value 2. What value is left at position 0 after the program halts?

//parameters modes:
//mode 0: position mode, the parameter to an instruction is interpreted as a position. e.g. if a
//    parameter is 50 it means its value is the value stored at address 50 in memory.
//mode 1: immediiate mode, a parameter is interpreted as a value, we do not need to read it from
//    an address.
//
//Parameter modes are stored in the same value as the instruction's opcode. 
//The opcode is a two-digit number based only on the ones and tens digit of the value, that is, the opcode is the rightmost two digits of the first value in an instruction. 
//Parameter modes are single digits, one per parameter, read right-to-left from the opcode: the first parameter's mode is in the hundreds digit, the second parameter's mode is in the thousands digit, the third parameter's mode is in the ten-thousands digit, and so on. 
//Any missing modes are 0

//5: is jump-if-true: if the first parameter is non-zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
//6: is jump-if-false: if the first parameter is zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
//7: is less than: if the first parameter is less than the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
//8: is equals: if the first parameter is equal to the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.

//However, if the instruction modifies the instruction pointer, that value is used and the instruction pointer is not automatically increased.


#[derive(Debug)]
enum Parameter {
    Position(usize),
    Immediate(i32)
}

#[derive(Debug)]
enum Instruction {
    Add(Parameter, Parameter, usize),
    Multiply(Parameter, Parameter, usize),
    Input(usize), // This one will always store 1 at the given address(usize)
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, usize),
    Equals(Parameter, Parameter, usize),
    Halt,
}

fn main() {
    let puzzle_input = fs::read_to_string(INFILE_PATH)
        .expect("Did not find {INFILE_PATH}");

    let program = parse_input(&puzzle_input);
   
    //let mut test_prog =vec![1001, 1, 2, 3];

    let permutations = all_phase_permutations(0, 5);

    let output_signals: Vec<Result<String, String>> = permutations.iter().map(|p| {
        println!("Running permutation: {:?}", p);
        let mut last_output = Err(format!("No outputs for permutation:{:?}", p));
        let mut prev_output_val = String::from("0");
        for phase_setting in p {
            last_output = run_program(&program, phase_setting.as_str(), prev_output_val.as_str());
            prev_output_val = last_output.clone()?;
        }
        return last_output 
    }).collect();
    let max_output_signal = output_signals.into_iter().flatten().flat_map(|s|  s.parse::<i32>()).max();
    println!("Part 1 max output signal: {:?}", max_output_signal);
   
}

fn parse_input(puzzle_input: &str) -> Vec<String> {
    puzzle_input.split(',').map(|x| x.to_owned()).collect()
}

fn all_phase_permutations(from: usize, to: usize) -> Vec<Vec<String>> {
    (from..to).map(|d| d.to_string()).permutations(to-from).unique().collect_vec()
}

//TODO: Could take inputs as a generic type of container
//TODO: should extract into a separate function that takes one input and returns first signal
fn run_program(program: &Vec<String>, phase_setting: &str, prev_output: &str) -> Result<String, String> {
    let mut program_copy = program.to_owned();

    let mut inputs = VecDeque::from([phase_setting, prev_output]);

    run_until_halt(&mut program_copy, &mut inputs)    
}

//TODO: Maybe take Input and Output handlers as arguments?
fn run_until_halt(program: &mut Vec<String>, inputs: &mut VecDeque<&str>) -> Result<String, String> {
    let mut last_output = Err(format!("Running program with {:?} did not give an output", inputs));

    let mut i = 0;

    while parse_next_instruction(&program, i).is_some()  {
        match run_instr(program, inputs, i) {
            Ok(InstrResult::NewInstrIndex(new_i)) => {
                i = new_i;
            }
            Ok(InstrResult::OutputAndNewInstrIndex(ProgramOutput(output), new_i)) => {
                i = new_i;
                last_output = Ok(output);
            }
            Ok(InstrResult::Halt) => {
                break;
            }
            Err(e) => {
                println!("{e}");
                break;
            }
        }
    }
    last_output
}

fn run_program_part_2(program: &Vec<String>) {
    let mut prev_output = String::from("");
    let mut next_input = String::from("0");

    let programs: Vec<_> = iter::repeat(program.clone()).zip((5..=9).map(|d| d.to_string())).collect();
    for (mut program, phase_setting) in programs {
        let mut i = 0;

        //let mut inputs = VecDeque::from([phase_setting, String::from(prev_output)]);
        let mut inputs = VecDeque::from([next_input.as_str()]);//TODO: fill this in
        while parse_next_instruction(&program, i).is_some()  {
            match run_instr(&mut program, &mut inputs, i) {
                Ok(InstrResult::NewInstrIndex(new_i)) => {
                    i = new_i;
                }
                Ok(InstrResult::OutputAndNewInstrIndex(ProgramOutput(output), new_i)) => {
                    i = new_i;
                    prev_output = output;
                }
                Ok(InstrResult::Halt) => {
                    break;
                }
                Err(e) => {
                    //println!("{e}");
                    break;
                }
            }
        }
        //println!("{:?}", prev_output);
    }
}


struct ProgramOutput(String);

enum InstrResult {
    NewInstrIndex(usize),
    OutputAndNewInstrIndex(ProgramOutput, usize),
    Halt
}

fn run_instr(program: &mut Vec<String>, inputs: &mut VecDeque<&str>, instr_index: usize) -> Result<InstrResult, String> {
    if let Some(instr) =  parse_next_instruction(&program, instr_index) {
        let handle_binary_op = |p1: Parameter, p2: Parameter, result_addr: usize, op: fn(i32, i32) -> String, prog: &mut Vec<String>| {
            let val1 = param_to_val(&prog, p1);
            let val2 = param_to_val(&prog, p2);
            
            if let Some(output_ref) = prog.get_mut(result_addr) {
                *output_ref = op(val1, val2);
            }
        };

        match instr {
            Instruction::Add(p1, p2, result_addr) => {
                handle_binary_op(p1, p2, result_addr, |x, y| (x + y).to_string(), program);
                return Ok(InstrResult::NewInstrIndex(instr_index + 4))
            }
            Instruction::Multiply(p1, p2, result_addr) => {
                handle_binary_op(p1, p2, result_addr, |x, y| (x * y).to_string(), program);
                return Ok(InstrResult::NewInstrIndex(instr_index + 4))
            }
            Instruction::Input(result_addr) => {
                if let (Some(output_ref), Some(inp)) = (program.get_mut(result_addr), inputs.pop_front()) {
                    *output_ref = String::from(inp);
                }
                return Ok(InstrResult::NewInstrIndex(instr_index + 2))
            }
            Instruction::Output(Parameter::Position(addr_to_print_from)) => {
                let output = program.get(addr_to_print_from).ok_or("Failed to get output from address {addr_to_print_from}")?;
                return Ok(InstrResult::OutputAndNewInstrIndex(ProgramOutput((*output).to_owned()), instr_index+2))
            }
            Instruction::Output(Parameter::Immediate(output)) => {
                return Ok(InstrResult::OutputAndNewInstrIndex(ProgramOutput(output.to_string()), instr_index+2));
            }
            Instruction::JumpIfTrue(p1, p2) => {
                if param_to_val(&program, p1) != 0 {
                    return usize::try_from(param_to_val(&program, p2))
                        .map_err(|_| format!("failed"))
                        .map(|v2| InstrResult::NewInstrIndex(v2))
                }
                return Ok(InstrResult::NewInstrIndex(instr_index+3))
            }
            Instruction::JumpIfFalse(p1, p2) => {//TODO: maybe this one should be same handler above but args flipped?
                if param_to_val(&program, p1) == 0 {
                    return usize::try_from(param_to_val(&program, p2))
                        .map_err(|_| format!("failed"))
                        .map(|v2| InstrResult::NewInstrIndex(v2))
                }
                return Ok(InstrResult::NewInstrIndex(instr_index+3))
            }
            Instruction::LessThan(p1, p2, result_addr) => {
                handle_binary_op(p1, p2, result_addr, |x, y| String::from(if x < y {"1"} else {"0"}), program);
                return Ok(InstrResult::NewInstrIndex(instr_index+4))
            }
            Instruction::Equals(p1, p2, result_addr) => {
                handle_binary_op(p1, p2, result_addr, |x, y| String::from(if x == y {"1"} else {"0"}), program);
                return Ok(InstrResult::NewInstrIndex(instr_index+4))
            }
            Instruction::Halt => {
                return Ok(InstrResult::Halt);
            }
        }
    } else {
        return Err(format!("Failed to to parse instr_index {instr_index}"));
    }
}


fn param_to_val(program: &Vec<String>, param: Parameter) -> i32 {
    match param {
        Parameter::Position(addr) => program.get(addr)
            .map_or(0, |v_ref| {
                let err_or_res = (*v_ref).trim().parse();
                err_or_res.expect("param_to_val") 
            }),
        Parameter::Immediate(v) => v,
    }
}

fn parse_next_instruction(program: &Vec<String>, start_index: usize) -> Option<Instruction> {
    let opcode_and_modes = program.get(start_index)?.to_string(); 

    let start_of_opcode_index = opcode_and_modes.char_indices().rev().nth(1);

    let opcode = &opcode_and_modes[start_of_opcode_index.unwrap_or((0, '0')).0..];
    match opcode {
        "01" | "1" | "02" | "2" | "07" | "7" | "08" | "8" => {
            if let Some(&[ref fst_param, ref snd_param, ref result_addr]) = program.get(start_index+1..=start_index+3) {
                if let (Ok(fst_param), Ok(snd_param)) = (fst_param.parse(), snd_param.parse()) {
                    let (fst_is_immediate, snd_is_immediate) = parse_modes(&opcode_and_modes);
                    let fst_param = parse_param(&fst_param, fst_is_immediate)?;
                    let snd_param = parse_param(&snd_param, snd_is_immediate)?;
                    let result_addr = result_addr.parse().ok()?;

                    if opcode == "01" || opcode == "1" {
                        return Some(Instruction::Add(fst_param, snd_param, result_addr))
                    } else if opcode == "02" || opcode == "2" {
                        return Some(Instruction::Multiply(fst_param, snd_param, result_addr))
                    } else if opcode == "07" || opcode == "7" {
                        return Some(Instruction::LessThan(fst_param, snd_param, result_addr))
                    } else if opcode == "08" || opcode == "8" {
                        return Some(Instruction::Equals(fst_param, snd_param, result_addr))
                    }
                }
            }
            return None
        },
        "03" | "3" => {
            program.get(start_index+1)
                .map(|addr| (*addr).parse().ok())
                .flatten()
                .map(|addr|  {Instruction::Input(addr)})},
        "04" | "4" => {
            let (_, immediate_mode) = parse_modes(&opcode_and_modes);
            if let Some(fst_param) = program.get(start_index+1).map(|s| s.parse().ok()).flatten() {
                return Some(Instruction::Output(parse_param(&fst_param, immediate_mode)?))
            }
            None
        },
        "05" | "5" | "06" | "6" => {
            if let Some(&[ref fst_param, ref snd_param]) = program.get(start_index+1..=start_index+2) {
                if let (Ok(fst_param), Ok(snd_param)) = (fst_param.parse(), snd_param.parse()) {
                    let (fst_is_immediate, snd_is_immediate) = parse_modes(&opcode_and_modes);
                    let fst_param = parse_param(&fst_param, fst_is_immediate)?;
                    let snd_param = parse_param(&snd_param, snd_is_immediate)?;
                    if opcode == "05" || opcode == "5" {
                        return Some(Instruction::JumpIfTrue(fst_param, snd_param))
                    } else {
                        return Some(Instruction::JumpIfFalse(fst_param, snd_param))
                    }
                }
            }
            None
        },
        "99" => Some(Instruction::Halt),
        _ => None
    }
}

fn parse_modes(opcode_and_modes :&String) -> (bool, bool) {
    let parse_mode_at_index = |i| opcode_and_modes.chars().nth(i).unwrap_or('0') == '1';
    match opcode_and_modes.len() {
        3 => (parse_mode_at_index(0), false),
        4 => (parse_mode_at_index(1), parse_mode_at_index(0)),
        _ => (false, false),
    }
}

fn parse_param(param: &String, immediate_mode: bool) -> Option<Parameter> {
    if let Ok(p) = param.parse() {
        if immediate_mode {
            return Some(Parameter::Immediate(p))
        } else {
            return Some(Parameter::Position(usize::try_from(p).ok()?))
        }
    }
    None
}
