use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin().lock();
    let input: Vec<String> = stdin.lines().map(|l| l.unwrap()).collect();
    println!("calibrations_1 = {}", calibrations_1(&input));
    println!("calibrations_2 = {}", calibrations_2(&input));
}

fn calibrations_1(input: &Vec<String>) -> u32 {
    const RADIX: u32 = 10;

    let get_value = |line: &String| -> u32 {
        let mut digits = line.chars().filter_map(|c| c.to_digit(RADIX));
        let first = digits.next().unwrap_or(0);
        let last = digits.next_back().unwrap_or(first);
        first * RADIX + last
    };

    input.iter().map(|line| get_value(&line)).sum()
}

fn calibrations_2(input: &Vec<String>) -> u32 {
    const RADIX: u32 = 10;
    const DIGITS_STR: [(&'static str, u32); 9] = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let get_value = |line: &String| -> u32 {
        let mut last_index = 0;
        let mut digits: Vec<u32> = Vec::new();
        for (index, char) in line.chars().enumerate() {
            if let Some(d) = char.to_digit(RADIX) {
                digits.push(d);
                last_index = index + 1;
            } else {
                let token = &line[last_index..index + 1];
                // dbg!(token);
                let digit = DIGITS_STR.iter().find(|&(s, _)| token.contains(s));
                match digit {
                    Some((_, d)) => {
                        digits.push(*d);
                        last_index = index;
                    }
                    None => continue,
                }
            }
        }
        // dbg!(&digits);
        let first = digits.first().unwrap_or(&0);
        let last = digits.last().unwrap_or(first);
        first * RADIX + last
    };

    input.iter().map(|line| get_value(&line)).sum()
}
