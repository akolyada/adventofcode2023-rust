use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin().lock();

    let mut input: Vec<(String, usize)> = stdin
        .lines()
        .filter_map(|s| s.ok())
        .map(|s| {
            s.split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .map(|v| (v[0].clone(), v[1].parse::<usize>().unwrap()))
        .collect();

    input.sort_unstable_by(|l, r| hand_type_sort(l.0.as_str(), r.0.as_str(), false));
    let winnings = input
        .iter()
        .enumerate()
        .fold(0, |acc, (n, val)| acc + (val.1 * (n + 1)));

    println!("Total winnings = {}", winnings);

    input.sort_unstable_by(|l, r| hand_type_sort(l.0.as_str(), r.0.as_str(), true));
    let winnings = input
        .iter()
        .enumerate()
        .fold(0, |acc, (n, val)| acc + (val.1 * (n + 1)));

    println!("Total winnings2 = {}", winnings);
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn get_hand_type(hand: &str) -> HandType {
    const HAND_LEN: usize = 5;
    assert_eq!(HAND_LEN, hand.chars().count());

    let mut card_frequency: HashMap<char, usize> = HashMap::new();
    for card in hand.chars() {
        card_frequency
            .entry(card)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut frequency: Vec<&usize> = card_frequency.values().collect();
    let freq_len = frequency.len();

    match freq_len {
        1 => HandType::FiveOfAKind,
        2 => {
            frequency.sort_unstable_by(|a, b| b.cmp(a));
            let first = frequency.first().unwrap();
            let result = if **first == 4 {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            };
            result
        }
        3 => {
            frequency.sort_unstable_by(|a, b| b.cmp(a));
            let first = frequency.first().unwrap();
            let result = if **first == 3 {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPair
            };
            result
        }
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        _ => panic!("Max hand length 5 is expected!"),
    }
}

fn get_hand_type_j(hand: &str) -> HandType {
    const HAND_LEN: usize = 5;
    assert_eq!(HAND_LEN, hand.chars().count());

    let mut card_frequency: HashMap<char, usize> = HashMap::new();
    let mut j_count = 0;
    for card in hand.chars() {
        if card != 'J' {
            card_frequency
                .entry(card)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        } else {
            j_count += 1;
        }
    }

    let mut frequency: Vec<usize> = card_frequency.values().cloned().collect();
    if !frequency.is_empty() {
        frequency.sort_unstable_by(|a, b| b.cmp(a));
        frequency[0] += j_count;
    }

    let freq_len = frequency.len();
    match freq_len {
        0..=1 => HandType::FiveOfAKind,
        2 => {
            let first = frequency.first().unwrap();
            let result = if *first == 4 {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            };
            result
        }
        3 => {
            let first = frequency.first().unwrap();
            let result = if *first == 3 {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPair
            };
            result
        }
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        _ => panic!("Max hand length 5 is expected!"),
    }
}

fn get_card_score(card: char) -> u8 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => 0,
    }
}

fn get_card_score_j(card: char) -> u8 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        'J' => 1,
        _ => 0,
    }
}

fn hand_type_sort(left_hand: &str, right_hand: &str, joker: bool) -> Ordering {
    let get_hand_type_fn = if joker {
        get_hand_type_j
    } else {
        get_hand_type
    };
    let get_card_score_fn = if joker {
        get_card_score_j
    } else {
        get_card_score
    };

    let ltype = get_hand_type_fn(left_hand);
    let rtype = get_hand_type_fn(right_hand);

    if ltype < rtype {
        return Ordering::Greater;
    }

    if ltype > rtype {
        return Ordering::Less;
    }

    let cards = left_hand.chars().zip(right_hand.chars());
    for (left_card, right_card) in cards {
        if left_card != right_card {
            return get_card_score_fn(left_card).cmp(&get_card_score_fn(right_card));
        }
    }

    Ordering::Equal
}
