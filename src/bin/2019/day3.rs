use std::fs;
use std::collections::HashSet;
use std::iter;

const INFILE_PATH: &str = "infiles/2019/day3.in";
const TEST_INPUT: &str = "R8,U5,L5,D3\nU7,R6,D4,L4";

#[derive(Debug)]
enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

fn main() {
    let puzzle_input = fs::read_to_string(INFILE_PATH)
        .expect("Did not find {INFILE_PATH}");
    //let directions = parse_input(&puzzle_input);
    let directions = parse_input(&TEST_INPUT.to_string());

    let fst = directions_to_coordinates(&directions[0]);
    let snd = directions_to_coordinates(&directions[1]);

    let fst_as_set: HashSet<(i32, i32)> = HashSet::from_iter(fst.clone());
    let snd_as_set = HashSet::from_iter(snd.clone());
    let intersecting_coords: HashSet<_> = fst_as_set.intersection(&snd_as_set)
        .filter(|&coord| *coord != (0, 0))
        .collect();

    let distances_to_start: Vec<i32> = intersecting_coords.clone().iter()
        .map(|&coord| manhattan_distance(*coord, (0, 0)))
        .collect();

    let closest_distance = distances_to_start.iter().min();
    println!("Part 1: {:?}", closest_distance);

    println!("1{:?}", directions[0]);
    println!("2{:?}", directions[1]);
    let combined_shortest_length = intersecting_coords.iter()
        .map(|&coord| {
            println!("coord:{:?}\nfst full path:{:?}\nsnd full path{:?}\nfst len:{:?}\nsnd len:{:?}\n", coord, fst, snd, length_to_intersection(&fst, *coord), length_to_intersection(&snd, *coord));
            length_to_intersection(&fst, *coord) + length_to_intersection(&snd, *coord)
        })
        .min();

    println!("Part 2: {:?}", combined_shortest_length);
}

fn parse_input(puzzle_input: &String) -> Vec<Vec<Direction>> {
    let mut results = Vec::new();

    let splitted_input = puzzle_input.split('\n');
    for line in splitted_input {
        let list_of_directions = line.split(',')
                                    .flat_map(|x| parse_direction(x))
                                    .collect::<Vec<Direction>>();
        results.push(list_of_directions);
    }

    results
}

fn parse_direction(dir: &str) -> Option<Direction> {
    let start = dir.chars();
    match start.skip(1).take(3).collect::<String>().parse() {
        Ok(length) => {
            match dir.chars().next()? {
                'U' => Some(Direction::Up(length)),
                'D' => Some(Direction::Down(length)),
                'L' => Some(Direction::Left(length)),
                'R' => Some(Direction::Right(length)),
                _ => None
            } 
        },
        Err(_) => { 
            None
        }
    }
}

fn directions_to_coordinates(directions: &Vec<Direction>) -> Vec<(i32, i32)> {
    let mut results = Vec::new();

    let mut coord = (0, 0);
    for dir in directions {
        let (x, y) = coord;

        let (x2, y2) = match dir {
            Direction::Up(dist)    => (x, y + dist),
            Direction::Down(dist)  => (x, y - dist),
            Direction::Left(dist)  => (x - dist, y),
            Direction::Right(dist) => (x + dist, y),
        };
        let coord_range: Vec<(i32, i32)> = match dir {
            Direction::Up(_)    => iter::repeat(x).zip(y..=y2).collect(),
            Direction::Down(_)  => iter::repeat(x).zip((y2..=y).rev()).collect(),
            Direction::Left(_)  => (x2..=x).rev().zip(iter::repeat(y)).collect(),
            Direction::Right(_) => (x..=x2).zip(iter::repeat(y)).collect(),
        };
        results.extend(&coord_range);

        coord = (x2, y2);
    }
    results
}

fn length_to_intersection(full_path: &Vec<(i32, i32)>, intersection: (i32, i32)) -> usize {
    full_path.iter()
        .filter(|&coord| *coord != (0, 0))
        .take_while(|&&x| x != intersection)
        .collect::<Vec<_>>()
        .len()
}

fn manhattan_distance((start_x, start_y): (i32, i32), (goal_x, goal_y): (i32, i32)) -> i32 {
    if goal_x == 0 && goal_y == 0 {
        return start_x.abs() + start_y.abs()
    }
    (start_x - goal_x).abs() + (start_y - goal_y).abs()
}
