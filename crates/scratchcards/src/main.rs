use std::collections::{HashMap, HashSet};
use std::io::prelude::*;
use std::{io, usize};

fn main() {
    let stdin = io::stdin().lock();

    let card_wins: Vec<_> = stdin
        .lines()
        .filter_map(|s| s.ok())
        .map(|s| {
            let mut input_numbers = s.split(":").last().unwrap().split("|");
            let win_numbers = input_numbers.next().unwrap();
            let card_numbers = input_numbers.next().unwrap();
            let win_numbers: HashSet<usize> = win_numbers
                .split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect();
            let card_numbers: HashSet<usize> = card_numbers
                .split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect();
            win_numbers.intersection(&card_numbers).count()
        })
        .enumerate()
        .collect();

    let points: usize = card_wins
        .iter()
        .filter_map(|&(_, count)| match count {
            0 => None,
            1 => Some(1),
            n => Some(usize::pow(2, (n as u32) - 1)),
        })
        .sum();

    println!("points = {:?}", points);

    let mut cards_copies: HashMap<usize, usize> = HashMap::new();
    for (card_num, wins) in card_wins {
        let add_count = cards_copies.entry(card_num).or_insert(1).to_owned();
        for n in card_num + 1..=card_num + wins {
            cards_copies
                .entry(n)
                .and_modify(|count| *count += add_count)
                .or_insert(add_count + 1);
        }
    }

    println!("cards = {:?}", cards_copies.values().sum::<usize>());
}
