use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

const INFILE_PATH: &str = "../infiles/2023/day11.in";

type Coordinate = (u64, u64);
type Set = HashSet<Coordinate>;

fn main() {
    let puzzle_input = fs::read_to_string(INFILE_PATH)
        .expect("Did not find {INFILE_PATH}");
    
    let galaxies = parse_galaxies(&puzzle_input);
    let expanded = expand_universe(&galaxies, 1);
    let shortest_paths = all_shortest_paths(&expanded);
    println!("{:?}", shortest_paths.into_iter().map(|v| v.1).sum::<u64>());

    let galaxies = parse_galaxies(&puzzle_input);
    let expanded = expand_universe(&galaxies, 999999);
    let shortest_paths = all_shortest_paths(&expanded);
    println!("{:?}", shortest_paths.into_iter().map(|v| v.1).sum::<u64>());
}

fn parse_galaxies(input: &str) -> Set {
    let mut result = Set::new();
    for (row, line) in input.split('\n').filter(|line| !line.is_empty()).enumerate() {
        for (col, _) in line.chars().enumerate().filter(|(_, c)| *c == '#') {
            result.insert((row as u64, col as u64));
        }
    }
    result
}

fn expand_universe(galaxies: &Set, expand_distance: u64) -> Set {
    let min_row = 0;
    let max_row = galaxies.iter().map(|(r, _)| *r).max().unwrap_or_default();
    let min_col = 0;
    let max_col = galaxies.iter().map(|(_, c)| *c).max().unwrap_or_default();

    let empty_rows: Vec<_> = (min_row..=max_row).filter(|r| !(min_col..=max_col).any(|c| galaxies.contains(&(*r, c)))).collect();
    let empty_cols: Vec<_> = (min_col..=max_col).filter(|c| !(min_row..=max_row).any(|r| galaxies.contains(&(r, *c)))).collect();

    let mut galaxy_move_counts: HashMap<_, _> = galaxies.iter().map(|coord| (coord, (0, 0))).collect();

    for empty_row in empty_rows.iter() {
        let galaxies_to_move: Vec<_> = galaxies.clone().into_iter().filter(|(row, _)| row > empty_row).collect();
        for coord in galaxies_to_move.into_iter() {
            if let Some((rc, _)) = galaxy_move_counts.get_mut(&coord) {
                *rc = *rc + expand_distance;
            }
        }
    }
    
    for empty_col in empty_cols.iter() {
        let galaxies_to_move: Vec<_> = galaxies.clone().into_iter().filter(|(_, col)| col > empty_col).collect();
        for coord in galaxies_to_move.into_iter() {
            if let Some((_, cc)) = galaxy_move_counts.get_mut(&coord) {
                *cc = *cc + expand_distance;
            }
        }
    }

    let mut result = Set::new();
    for (r, c) in galaxies.iter() {
        if let Some((add_r, add_c)) = galaxy_move_counts.get(&(*r, *c)) {
            result.insert((r+add_r, c+add_c));
        }
    }
    result
}

fn all_shortest_paths(galaxies: &Set) -> HashSet<((Coordinate, Coordinate), u64)> {
    let all_pairs: HashSet<(Coordinate, Coordinate)> = galaxies.iter()
        .flat_map(|g1| galaxies.iter()
                  .map(|g2| (*g1, *g2))
                  .collect::<Vec<_>>())
        .collect();
    let mut pairs = HashSet::new();
    for pair@(g1, g2) in all_pairs {
        if !pairs.contains(&(g2, g1)) {
            pairs.insert(pair);
        }
    }

    pairs.iter().map(|p@((r1, c1), (r2, c2))| (*p, r1.abs_diff(*r2) + c1.abs_diff(*c2))).collect()
}




























