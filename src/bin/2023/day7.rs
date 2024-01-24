use std::{fs, collections::HashMap, cmp};
use itertools::Itertools;

const INFILE_PATH: &str = "../infiles/2023/day7.in";

//1. parse (hand, bid)'s into Vec
//2. sort the Vec based on hand rules
//3. sum up bid's with ranks

// Default ord derivings go from top to bottom :)
#[derive(PartialEq, Eq, PartialOrd, Hash, Ord, Debug, Copy, Clone)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
enum Hand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse, 
    FourOfAKind,
    FiveOfAKind
}

fn main() {
    let puzzle_input = fs::read_to_string(INFILE_PATH)
        .expect("Did not find {INFILE_PATH}");
    
    let parsed = parse_puzze_input(&puzzle_input, false);
    let ranked_hands = rank_hands(parsed);
    println!("{:?}", ranked_hands.iter().map(|(rank, bet)| (*rank+1) as u32 * bet).sum::<u32>());
    let parsed = parse_puzze_input(&puzzle_input, true);
    let ranked_hands = rank_hands(parsed);
    println!("{:?}", ranked_hands.iter().map(|(rank, bet)| (*rank+1) as u32 * bet).sum::<u32>());
}


fn parse_puzze_input(input: &str, use_jokers: bool) -> Vec<((Hand, [Card; 5]), u32)> {
    let mut result = Vec::new();

    for line in input.split_terminator('\n')  {
        if let [hand, bet] = line.split_whitespace().collect::<Vec<_>>().as_slice() {
            if let Ok(b) = bet.parse() {
                result.push((parse_hand(hand, use_jokers), b));
            }
        }
    }
    result
}

fn parse_hand(hand_str: &str, use_jokers: bool) -> (Hand, [Card; 5]) {
    let mut cards = [Card::Two; 5];
    if let [c1, c2, c3, c4, c5] = hand_str.chars().map(|c| parse_card(c, use_jokers)).collect::<Vec<Card>>()[..] {
        cards = [c1, c2, c3, c4, c5];
    }

    let mut countings = HashMap::new();
    for c in hand_str.chars() {
        let c = parse_card(c, use_jokers);
        if let Some(count) = countings.get(&c) {
            countings.insert(c, count+1);
        } else {
            countings.insert(c, 1);
        }
    }

    let mut hand = match countings.iter().sorted_by_key(|x| x.1).rev().collect::<Vec<_>>().as_slice() {
        [(_, _)] => Hand::FiveOfAKind,
        [(_, 4), (_, _)] => Hand::FourOfAKind,
        [(_, 3), (_, _)] => Hand::FullHouse,
        [(_, 3), (_, _), (_, _)] => Hand::ThreeOfAKind,
        [(_, 2), (_, 2), (_, _)] => Hand::TwoPair,
        [(_, 2), (_, _), (_, _), (_, _)] => Hand::OnePair,
        _ => Hand::HighCard,
    };

    if use_jokers && countings.contains_key(&Card::Joker) {
        hand = match countings.len() {
            2 => Hand::FiveOfAKind,
            3 => {
                if countings.get(&Card::Joker).is_some_and(|n| *n == 1)
                    && countings.values().contains(&2) {
                    Hand::FullHouse
                } else {
                    Hand::FourOfAKind
                }
            },
            4 => Hand::ThreeOfAKind,
            5 => Hand::OnePair,
            _ => Hand::FiveOfAKind
        };
    }

    (hand, cards)
}

fn parse_card(c: char, use_jokers: bool) -> Card {
    match c {
        'A' => Card::Ace,
        'K' => Card::King,
        'Q' => Card::Queen,
        'J' => {if use_jokers {Card::Joker} else {Card::Jack}},
        'T' => Card::Ten,
        '9' => Card::Nine,
        '8' => Card::Eight,
        '7' => Card::Seven,
        '6' => Card::Six,
        '5' => Card::Five,
        '4' => Card::Four,
        '3' => Card::Three,
        '2' => Card::Two,
        _ => panic!("Invalid card char {c}")
    }
}

fn cmp_hands((h1, cs1): (Hand, [Card; 5]), (h2, cs2): (Hand, [Card; 5])) -> cmp::Ordering {
    if h1 == h2 {
        // lexicographic comparison
        return cs1.cmp(&cs2) 
    }
    h1.cmp(&h2)
}

fn rank_hands(hands: Vec<((Hand, [Card; 5]), u32)>) -> Vec<(usize, u32)> {
    let mut hands_clone = hands.clone();
    hands_clone.sort_by(|(h1, _), (h2, _)| cmp_hands(*h1, *h2));
    hands_clone.iter().map(|(_, bet)| *bet).enumerate().collect()
}
