use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

const INFILE_PATH: &str = "infiles/2019/day6.in";

const CENTER_OF_MASS: &str = "COM";
const EXAMPLE_INPUT: &str = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
/* 1. make set of keys in orbit & store all possible object in set (any object that is not in key-set but in set is
 *    final node) & make multimap for traversing
 * 2. Start from final node in multimap, recursibely follow links, counting links as you go
 * */

fn main() {
    let puzzle_input = fs::read_to_string(INFILE_PATH)
        .expect("Did not find {INFILE_PATH}");

    let (in_orbits_to_orbitted, starts) = parse_input(EXAMPLE_INPUT);
    println!("{}", count_orbits(&in_orbits_to_orbitted, &starts));
}

fn parse_input(puzzle_input: &str) -> (HashMap<&str, &str>, HashSet<&str>) {
    let lines = puzzle_input.lines();

    let mut in_orbits_to_orbitted = HashMap::new();
    let mut orbitted_around = HashSet::new();
    let mut all_objects = HashSet::new();

    for line in lines.into_iter() {
        if let Some((orbitted, in_orbit)) = line.split_once(')') {
            in_orbits_to_orbitted.insert(in_orbit, orbitted);
            orbitted_around.insert(orbitted);
            all_objects.extend([orbitted, in_orbit]);
        }
    }

    let mut starts = HashSet::new();
    starts.extend(all_objects.difference(&orbitted_around));
    (in_orbits_to_orbitted, starts)
}

fn count_orbits(in_orbits_to_orbitted: &HashMap<&str, &str>, starts: &HashSet<&str>) -> u32 {
    let mut total_count = 0;

    for start in starts {
        println!("{total_count}");
        total_count += count(start, &in_orbits_to_orbitted);
    }
    total_count
}
//TODO: current bug is that you are counting twice from the common parts of the paths
//One solution is to keep track of visited nodes, maybe a good idea to make a struct actually 
fn count(from: &str, in_orbits_to_orbitted: &HashMap<&str, &str>) -> u32 {
    let mut count = 0;
    let mut curr = from;
    let mut directs_count = 0;
    while let Some(next) = in_orbits_to_orbitted.get(curr) {
        println!("{count}");
        count += directs_count + 1;
        directs_count += 1;
        curr = next;
    }
    count
}

