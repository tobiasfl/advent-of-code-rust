use std::fs;

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

//in part 2 the input instruction should get 5

#[derive(Debug)]
enum Parameter {
    Position(usize),
    Immediate(i32)
}

#[derive(Debug)]
enum Instruction {
    Add(Parameter, Parameter, usize),
    Multiply(Parameter, Parameter, usize),
    Input(usize),
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


    run_program(&program, "1");
    run_program(&program, "5");
}

fn parse_input(puzzle_input: &str) -> Vec<String> {
    puzzle_input.split(',').map(|x| x.to_owned()).collect()
}

fn run_program(program: &Vec<String>, input_instr_param: &str) {
    let mut program_copy = program.to_owned();

    let mut i = 0;
    while let Some(instr) = parse_next_instruction(&program_copy, i)  {
        //println!("{:?}", instr);
        match instr {
            Instruction::Add(p1, p2, result_addr) => {
                let val1 = param_to_val(&program_copy, p1);
                let val2 = param_to_val(&program_copy, p2);
                
                if let Some(output_ref) = program_copy.get_mut(result_addr) {
                    *output_ref = (val1 + val2).to_string();
                }
                i += 4;
            },
            Instruction::Multiply(p1, p2, result_addr) => {
                let val1 = param_to_val(&program_copy, p1);
                let val2 = param_to_val(&program_copy, p2);
                
                if let Some(output_ref) = program_copy.get_mut(result_addr) {
                    *output_ref = (val1 * val2).to_string();
                }
                i += 4;
            },
            Instruction::Input(result_addr) => {
                if let Some(output_ref) = program_copy.get_mut(result_addr) {
                    *output_ref = String::from(input_instr_param); 
                }
                i += 2;
            },
            Instruction::Output(Parameter::Position(addr_to_print_from)) => {
                if let Some(to_print_ref) = program_copy.get_mut(addr_to_print_from) {
                    println!("Output={}", *to_print_ref);
                }
                i += 2;
            },
            Instruction::Output(Parameter::Immediate(val_to_print)) => {
                println!("Output={val_to_print}");
                i += 2;
            },
            Instruction::JumpIfTrue(p1, p2) => {
                if param_to_val(&program_copy, p1) != 0 {
                    if let Some(val2) = usize::try_from(param_to_val(&program_copy, p2)).ok() {
                        i = val2;
                    }
                } else {
                    i += 3;
                }
            },
            Instruction::JumpIfFalse(p1, p2) => {
                if param_to_val(&program_copy, p1) == 0 {
                    if let Some(val2) = usize::try_from(param_to_val(&program_copy, p2)).ok() {
                        i = val2;
                    }
                } else {
                    i += 3;
                }
            },
            Instruction::LessThan(p1, p2, result_addr) => {
                let val1 = param_to_val(&program_copy, p1);
                let val2 = param_to_val(&program_copy, p2);
                
                if let Some(output_ref) = program_copy.get_mut(result_addr) {
                    *output_ref = (if val1 < val2 {"1"} else {"0"}).to_string();
                }
                
                i += 4;
            },
            Instruction::Equals(p1, p2, result_addr) => {
                let val1 = param_to_val(&program_copy, p1);
                let val2 = param_to_val(&program_copy, p2);
                
                if let Some(output_ref) = program_copy.get_mut(result_addr) {
                    *output_ref = (if val1 == val2 {"1"} else {"0"}).to_string();
                }
                
                i += 4;
            },
            Instruction::Halt => {
                println!("Halt");
                break;
            }
        }
        //println!("prog index:{i}");
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
    //println!("parse opcode and modes {opcode_and_modes}");

    let start_of_opcode_index = opcode_and_modes.char_indices().rev().nth(1);

    let opcode = &opcode_and_modes[start_of_opcode_index.unwrap_or((0, '0')).0..];
    //println!("parse opcode {opcode}");
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
