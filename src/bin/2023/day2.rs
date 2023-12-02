use std::{fs, num::ParseIntError};

const INFILE_PATH: &str = "../infiles/2023/day2.in";

type GameId = u32;
type Game = (GameId, Vec<Round>);

#[derive(Debug)]
struct Round {
    green: u32,
    blue: u32,
    red: u32
}

fn main() {
    let puzzle_input = fs::read_to_string(INFILE_PATH)
        .expect("Did not find {INFILE_PATH}");
   
    let all_games = parse_games(&puzzle_input);
    //let possible_games = find_possible_games(all_games, 12, 14, 13);
    //let sum_of_ids = possible_games.iter()
    //    .map(|(id, _)| *id)
    //    .reduce(|acc, e| acc + e);
    //println!("{:?}", sum_of_ids);

    let minimum_sets_of_cubes: Vec<(u32, u32, u32)> = all_games.iter()
        .map(|(_, rounds)| 
             (rounds.into_iter()
                  .max_by_key(|r| r.red)
                  .expect("empty rounds")
                  .red,
              rounds.into_iter()
                  .max_by_key(|r| r.green)
                  .expect("empty rounds")
                  .green,
              rounds.into_iter()
                  .max_by_key(|r| r.blue)
                  .expect("empty rounds")
                  .blue,
              ))
        .collect();
    let sum_of_powers = minimum_sets_of_cubes.iter()
        .map(|(r, g, b)| r * g * b)
        .reduce(|acc, p| acc + p);
    println!("{:?}", sum_of_powers);
}

fn find_possible_games(games: Vec<Game>, max_r: u32, max_b: u32, max_g: u32) -> Vec<Game> {
    games.into_iter()
        .filter(|(_, rounds)| 
                    !rounds.iter().any(|r| r.green > max_g || r.red > max_r || r.blue > max_b))
        .collect::<Vec<Game>>()

}

fn parse_games(input: &str) -> Vec<Game>  {
    let mut result = Vec::new();
    for line in input.split('\n').filter(|s| !s.is_empty()) {
        let game_id = line.chars()
            .skip(5)
            .take_while(|&c| c != ':')
            .collect::<String>()
            .parse()
            .expect("Failed to parse game id");


        let rounds_str = line.chars()
            .skip_while(|&c| c != ':')
            .skip(1)
            .collect();
        let games = parse_rounds(rounds_str);

        result.push((game_id, games));
    }


    result
}

fn parse_rounds(rounds_input_only: String) -> Vec<Round> {
    let mut result = Vec::new();
    for round in rounds_input_only.split(';') {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
       
        let colors = round
            .split(',')
            .map(|s| s.split_whitespace().collect::<Vec<&str>>());

        for color_and_n in colors {
            if let Some(v) = color_and_n.get(0) {
                if let Ok(n) = v.parse::<u32>() {
                    if let Some(color) = color_and_n.get(1) {
                        match *color {
                            "green" => green += n,
                            "red" => red += n,
                            "blue" => blue += n,
                            _ => {}
                        }
                    }
                }
            }
        }
        result.push(Round { green, blue, red })
    }
    result
}
















