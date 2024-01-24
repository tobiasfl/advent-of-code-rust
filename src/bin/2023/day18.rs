use std::fs;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::iter;

const INFILE_PATH: &str = "../infiles/2023/day18ex.in";

#[derive(Debug)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

//type Color = (u8, u8, u8);
type Color = String;
type Instruction = (Dir, u8, Color);

type Coordinate = (i32, i32);

//TODO: give up current futile attempt and use shoelace fromula
//or maybe pick's theorem instead

fn main() {
    let puzzle_input = fs::read_to_string(INFILE_PATH)
        .expect("Did not find {INFILE_PATH}");
    
    let instructions = parse_puzzle_input(&puzzle_input).unwrap(); 
    
    let digged = digged_edges2(&instructions);
    println!("{:?}", digged);
    println!("{:?}", digged.len());
    let area = shoelace_formula_area(&digged);

    println!("{:?}", area);
}

fn parse_puzzle_input(input: &str) -> Option<Vec<Instruction>> {
    let mut result = Vec::new();
    for line in input.split_terminator('\n') {
        let splitted: Vec<&str> = line.split_whitespace().collect();
        let dir = match splitted.first() {
            Some(&"L") => Some(Dir::Left),
            Some(&"R") => Some(Dir::Right),
            Some(&"U") => Some(Dir::Up),
            Some(&"D") => Some(Dir::Down),
            _ => None
        }?;
        let len: u8 = splitted.get(1).and_then(|l| l.parse().ok())?;
        let color = splitted.get(2)?;
        result.push((dir, len, String::from(*color)));
    }

    Some(result)
}

fn digged_edges2(instrs: &Vec<Instruction>) -> Vec<Coordinate> {
    let mut result = Vec::new();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    // Start index is added last
    for (dir, len, _) in instrs.into_iter() {
        match dir {
            Dir::Up => {
                let new_y = y + i32::from(*len);
                result.push((x, new_y));
                y = new_y;
            },
            Dir::Down => {
                let new_y = y - i32::from(*len);
                result.push((x, new_y));
                y = new_y;
            },
            Dir::Right => {
                let new_x = x + i32::from(*len);
                result.push((new_x, y));
                x = new_x;
            },
            Dir::Left => {
                let new_x = x - i32::from(*len);
                result.push((new_x, y));
                x = new_x;
            }
        }
    }
    result
}

fn digged_edges(instrs: &Vec<Instruction>) -> Vec<Coordinate> {
    let mut result = Vec::new();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    // Start index is added last
    for (dir, len, _) in instrs.into_iter() {
        match dir {
            Dir::Up => {
                let new_y = y + i32::from(*len);
                result.extend(iter::repeat(x).zip(y+1..=new_y));
                y = new_y;
            },
            Dir::Down => {
                let new_y = y - i32::from(*len);
                result.extend(iter::repeat(x).zip((new_y..y).rev()));
                y = new_y;
            },
            Dir::Right => {
                let new_x = x + i32::from(*len);
                result.extend((x+1..=new_x).zip(iter::repeat(y)));
                x = new_x;
            },
            Dir::Left => {
                let new_x = x - i32::from(*len);
                result.extend((new_x..x).zip(iter::repeat(y)));
                x = new_x;
            }
        }
    }
   
    result
}

fn shoelace_formula_area(edges: &Vec<Coordinate>) -> Option<f64> {
    let mut area = 0.0;

    for i in 0..edges.len()-1 {
        if let Some([(x, y), (x1, y1)]) = edges.get(i..=i+1) {
            area = area + (x * y1 - x1 * y) as f64;
        } else {
            return None;
        }
    }
    
    if let (Some((x0, y0)), Some((xlast, ylast))) = (edges.first(), edges.last()) {
        area = area + (xlast * y0 - x0 * ylast) as f64;
    } else {
        return None;
    }

    Some(area.abs() / 2.0)
}
