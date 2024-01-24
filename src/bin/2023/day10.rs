use std::fs;
use std::collections::HashMap;

const INFILE_PATH: &str = "../infiles/2023/day10.in";

/*
    | is a vertical pipe connecting north and south.
    - is a horizontal pipe connecting east and west.
    L is a 90-degree bend connecting north and east.
    J is a 90-degree bend connecting north and west.
    7 is a 90-degree bend connecting south and west.
    F is a 90-degree bend connecting south and east.
    . is ground; there is no pipe in this tile.
    S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
*/

type Coordinate = (i32, i32);
type Map = HashMap<Coordinate, char>;

fn main() {
    let puzzle_input = fs::read_to_string(INFILE_PATH)
        .expect("Did not find {INFILE_PATH}");
    
    let map = parse_map(&puzzle_input);
    
    let (start_coord, _) = map.iter().find(|(_, val)| **val == 'S').unwrap();
    let path = get_loop_path(*start_coord, &map);
    println!("{:?}", path.len() / 2);
    
    println!("{:?}", polygon_area(&path));
}

fn parse_map(input: &str) -> Map {
    let mut result = HashMap::new();
    for (i, line) in input.split('\n').filter(|line| !line.is_empty()).enumerate() {
        for (j, c) in line.chars().enumerate() {
            result.insert((i as i32, j as i32), c);
        }
    }
    result
}

fn get_loop_path(start@(start_r, start_c): Coordinate, map: &Map) -> Vec<Coordinate> {
    let mut curr_coord = start;
    let mut next_coord = [(start_r - 1, start_c), (start_r + 1, start_c), (start_r, start_c - 1), (start_r, start_c + 1)]
        .into_iter().find(|c| is_connected(start, *c, map)).expect("BUG: no start direction connected");

    let mut curr_path = Vec::new();

    while map.get(&next_coord) != Some(&'S') {
        curr_path.push(curr_coord);
        curr_coord = next_coord;
      
        next_coord = [(curr_coord.0 - 1, curr_coord.1), (curr_coord.0 + 1, curr_coord.1), (curr_coord.0, curr_coord.1 - 1), (curr_coord.0, curr_coord.1 + 1)]
            .into_iter().find(|c| is_connected(curr_coord, *c, map) && Some(c) != curr_path.last()).expect("found no next coord");
    }

    curr_path.push(curr_coord);

    curr_path
}

fn is_connected(from@(from_r, from_col): Coordinate, to@(to_r, to_col): Coordinate, map: &Map) -> bool {
    let south_connecteds = ['7', 'F', '|', 'S'];
    let north_connecteds = ['L', 'J', '|', 'S'];
    let east_connecteds = ['L', 'F', '-', 'S'];
    let west_connecteds = ['7', '-', 'J', 'S'];

    if let (Some(from_c), Some(to_c)) = (map.get(&from), map.get(&to)) {
        if to_r == from_r - 1 {
            return north_connecteds.contains(from_c) && south_connecteds.contains(to_c)
        } else if to_r == from_r + 1 {
            return south_connecteds.contains(from_c) && north_connecteds.contains(to_c)
        } else if to_col == from_col + 1 {
            return east_connecteds.contains(from_c) && west_connecteds.contains(to_c)
        } else {
            return west_connecteds.contains(from_c) && east_connecteds.contains(to_c)
        }
    }
    false
}

// shoelace formula / Pick's theorem
fn polygon_area(path: &Vec<Coordinate>) -> i32 {
    let mut area = 0;

    for i in 0..path.len()-1 {
        if let Some([(x1, y1), (x2, y2)]) = path.get(i..=i+1) {
            area = area + (y1 + y2) * (x2 - x1) as i32;
        }   
    }
    
    (area.abs() - i32::try_from(path.len()).unwrap() + 3) / 2
}
