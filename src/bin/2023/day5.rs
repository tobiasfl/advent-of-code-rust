use std::fs;
use std::cmp;
use std::slice;

const INFILE_PATH: &str = "../infiles/2023/day5ex.in";

fn main() {
    let puzzle_input = fs::read_to_string(INFILE_PATH)
        .expect("Did not find {INFILE_PATH}");
    let (seeds, maps) = parse_puzze_input(&puzzle_input);

    let locations: Vec<u64> = seeds.iter().map(|s| find_location(*s, &maps)).collect();
    println!("{:?}", locations.iter().min());

    let locations: Vec<u64> = seeds.as_slice().chunks(2).flat_map(|chunk| 
        if let &[seed_start, len] = chunk {
            find_locations((seed_start, len), &maps)
        } else {
            vec![]
        })
        .collect();
    println!("{:?}", locations.iter().min());
}

fn parse_puzze_input(input: &str) -> (Vec<u64>, [Vec<(u64, u64, u64)>; 7]) {
    let splitted_on_paragraphs: Vec<&str> = input.split("\n\n").collect();

    let seeds = splitted_on_paragraphs
        .first()
        .map(|line| line.split(' ').flat_map(|n| n.parse().ok()).collect::<Vec<u64>>())
        .unwrap_or_default();
    let seed_to_soil = parse_map(splitted_on_paragraphs.get(1).unwrap_or(&""));
    let soil_to_fertilizer = parse_map(splitted_on_paragraphs.get(2).unwrap_or(&""));
    let fertilizer_to_water = parse_map(splitted_on_paragraphs.get(3).unwrap_or(&""));
    let water_to_light = parse_map(splitted_on_paragraphs.get(4).unwrap_or(&""));
    let light_to_temperature = parse_map(splitted_on_paragraphs.get(5).unwrap_or(&""));
    let temperature_to_humidity = parse_map(splitted_on_paragraphs.get(6).unwrap_or(&""));
    let humidity_to_location = parse_map(splitted_on_paragraphs.get(7).unwrap_or(&""));

    (seeds, [seed_to_soil, soil_to_fertilizer, fertilizer_to_water, water_to_light, light_to_temperature, temperature_to_humidity, humidity_to_location])
}

fn parse_map(range_lines: &str) -> Vec<(u64, u64, u64)> {
    let mut result = Vec::new();
    for line in range_lines.split_terminator('\n') {
        let nums = line.split_whitespace().flat_map(|n| n.parse::<u64>()).collect::<Vec<_>>();
        if let &[dst_range_start, src_range_start, len] = nums.as_slice() {
            result.push((dst_range_start, src_range_start, len));
        }
    }
    result
}
//TODO: do something clever to shorten this stuff (might also be able to calculate a seed range in
//constant time)
fn find_location(seed: u64, maps: &[Vec<(u64, u64, u64)>; 7]) -> u64 {
    let mut key = seed; 
    for map in maps {
        let map_entry = map.iter().find(|(_, src_start, len)| key >= *src_start && key < src_start + len);
        if let Some((dst_start, src_start, _)) = map_entry {
            key = dst_start + (key - src_start);
        }
    }
    key
}


//TODO: something wrong, does not work on actual input
fn find_locations((seed_start, seed_len): (u64, u64), maps: &[Vec<(u64, u64, u64)>; 7]) -> Vec<u64> {
    println!("checking {seed_start} {seed_len}");
    let mut locations = Vec::new();

    let mut key_start = seed_start;
    let mut key = key_start;

    let mut key_offset = seed_len;

    while key_start < seed_start + seed_len {
        for map in maps {
            let map_entry = map.iter().find(|(_, src_start, len)| key >= *src_start && key < src_start + len);
            if let Some((dst_start, src_start, len)) = map_entry {
                key_offset = cmp::min(key_offset, *len);
                key = dst_start + (key - src_start);
            }
        }
        locations.push(key);
        key_start = key_start + key_offset;
    }

    locations
}
