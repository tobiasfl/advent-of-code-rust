use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use itertools::Itertools;
use std::iter;

const INFILE_PATH: &str = "../infiles/2023/day3.in";

type Position = (usize, usize);
type NumberPosition = (Position, Position);
type Grid = Vec<Vec<char>>;

fn main() {
    let puzzle_input = fs::read_to_string(INFILE_PATH)
        .expect("Did not find {INFILE_PATH}");

    let grid = parse_grid(&puzzle_input);
    let part_numbers = find_part_numbers(&grid);
    let part_numbers_sum = part_numbers.into_iter().reduce(|acc, e| acc + e);
    println!("{:?}", part_numbers_sum);

    let gear_ratios = find_gear_ratios(&grid);
    let gear_ratios_sum = gear_ratios.into_iter().reduce(|acc, e| acc + e);
    println!("{:?}", gear_ratios_sum);
}

fn find_part_numbers(grid: &Grid) -> Vec<u32> {
    let number_positions = find_number_positions(&grid);
    let mut result = vec![];
    for num_pos in number_positions {
        if is_part_number(grid, num_pos) {
            if let Some(num) = number_position_to_number(grid, num_pos) {
                result.push(num);
            }
        }
    }

    result
}

fn is_part_number(grid: &Grid, ((s_r, s_c), (_, e_c)): NumberPosition) -> bool {
    for pos in iter::repeat(s_r).zip(s_c..=e_c) {
        for (nr, nc) in position_neighbours(pos) {
            if let Some(row) = grid.get(nr) {
                if row.get(nc).is_some_and(|v| !(v.is_digit(10) || *v == '.')) {
                    return true;
                }
            }
        }
    }
    false
}

fn position_neighbours((r, c): Position) -> Vec<Position> {
    let mut neighbors = vec![(r, c+1), (r+1, c), (r+1, c+1)];
    if r != 0 {
        neighbors.append(&mut vec![(r-1, c), (r-1, c+1)]);
    }
    if c != 0 {
        neighbors.append(&mut vec![(r, c-1), (r+1, c-1)]);
    }
    if c != 0 && r != 0 {
        neighbors.push((r-1, c-1));
    }
    neighbors
}

fn number_position_to_number(grid: &Grid, ((s_r, s_c), (_, e_c)): NumberPosition) -> Option<u32> {
    grid.get(s_r)
        .unwrap_or(&vec![])
        .get(s_c..=e_c) 
        .unwrap_or(&vec![])
        .iter()
        .collect::<String>()
        .parse::<u32>()
        .ok()
}

fn find_gear_ratios(grid: &Grid) -> Vec<u32> {
    let digit_to_n_pos = create_digit_to_number_pos_mapping(&grid);
    let potential_gears = find_potential_gear_positions(&grid);

    let mut result = vec![];
    for g_pos in potential_gears {
        let mut adjacent_nums =  HashSet::new();
        for pos in position_neighbours(g_pos) {
            if let Some(n_pos) = digit_to_n_pos.get(&pos) {
                adjacent_nums.insert(n_pos);
            }
        }
        if adjacent_nums.len() == 2  {
            result.push(adjacent_nums.iter().fold(1, |acc, e| acc * number_position_to_number(grid, **e).unwrap_or(0)))
        }
    }
    
    result
}

fn parse_grid(input: &str) -> Grid {
    input.split_whitespace()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect_vec())
        .collect()
}

fn create_digit_to_number_pos_mapping(grid: &Grid) -> HashMap<Position, NumberPosition> {
    let mut result = HashMap::new();
    let number_positions = find_number_positions(grid);
    for num_pos@((s_r, s_c), (_, e_c)) in number_positions {
        for pos in iter::repeat(s_r).zip(s_c..=e_c) {
            result.insert(pos, num_pos);
        }
    }
    result
}

fn find_number_positions(grid: &Grid) -> Vec<NumberPosition> {
    let mut result = vec![];
    for (i, row) in grid.iter().enumerate() {
        let mut num_start = None;
        let mut num_end = None;
        for (j, c) in row.iter().enumerate() {
            if let Some(_) = c.to_digit(10) {
                if num_start.is_none() {
                    num_start = Some((i, j));
                }                
                num_end = Some((i, j));
            } else if let (Some(s), Some(e)) = (num_start, num_end)  {
                    result.push((s, e));
                    num_start = None; 
                    num_end = None; 
            }
        }
        if let (Some(s), Some(e)) = (num_start, num_end)  {
            result.push((s, e));
        }
    }
    result
}

fn find_potential_gear_positions(grid: &Grid) -> Vec<Position> {
    let mut result = vec![];
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '*' {
                result.push((i, j));
            }        
        }
    }
    result
}
