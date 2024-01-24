use std::fs;
use std::iter;

const INFILE_PATH: &str = "../infiles/2023/day4.in";

type Card = (Vec<u32>, Vec<u32>);
type Cards = Vec<Card>;

fn main() {
    let puzzle_input = fs::read_to_string(INFILE_PATH)
        .expect("Did not find {INFILE_PATH}");

    let cards = parse_cards(&puzzle_input);
    let sum_of_points = find_all_winners(&cards).iter()
        .filter(|nums| !nums.is_empty())
        .map(|nums| usize::pow(2, nums.len() as u32) / 2)
        .reduce(|acc, e| acc + e);
    println!("{:?}", sum_of_points);

    let total_scratchcards = total_scratchcards(&cards);
    println!("{:?}", total_scratchcards);
}

fn parse_cards(input: &str) -> Cards {
    let mut result = Vec::new();
    for line in input.split('\n') {
        let mut sides: Vec<Vec<u32>> = Vec::new();
        for side in line.split('|') {
            sides.push(side.trim().split_whitespace().flat_map(|n| n.parse()).collect());
        }
        if let Some(&[ref l, ref r]) = sides.get(0..=1) {
            result.push((l.clone(), r.clone()));
        }
    }

    result
}

fn find_all_winners(cards: &Cards) -> Vec<Vec<u32>> {
    cards.into_iter()
        .map(|(win_nums, my_nums)| 
             my_nums.clone().into_iter().filter(|n| win_nums.contains(n)).collect()
         ).collect()
}

fn total_scratchcards(cards: &Cards) -> usize {
    let mut copy_counts: Vec<usize> = iter::repeat(1).take(cards.len()).collect();
    let winner_nums: Vec<usize> = find_all_winners(cards).iter().map(|c| c.len()).collect();
  
    for i in 0..copy_counts.len() {
        let card_wins = *winner_nums.get(i).unwrap();
        let curr_copy_count = *copy_counts.get(i).unwrap();
        for later_card_index in i+1..i+1+card_wins {
            if let Some(n) = copy_counts.get_mut(later_card_index) {
                *n += curr_copy_count;
            }
        }
         
    }
    copy_counts.into_iter().reduce(|acc, e| acc + e).unwrap()
}
