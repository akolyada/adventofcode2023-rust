use std::collections::HashMap;
use std::error::Error;
use std::io::prelude::*;
use std::str;
use std::{io, usize};

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin().lock();

    let mut width = 0;
    let mut input: Vec<u8> = Vec::new();
    for line in stdin.lines() {
        let line = line?;
        if width == 0 {
            width = line.len();
        }
        assert!(line.is_ascii());
        input.append(line.into_bytes().as_mut());
    }

    let mut parts: Vec<(&[u8], Option<usize>)> = Vec::new();
    let mut start_digit_pos = 0;
    let mut is_part_number = false;
    let mut star_adjacent = None;
    for (i, b) in input.iter().enumerate() {
        if b.is_ascii_digit() {
            if start_digit_pos == 0 {
                start_digit_pos = i;
            }
            if let Some(pos) = adjacent_to_symbol(input.as_slice(), width, i) {
                is_part_number = true;
                if input[pos] == b'*' {
                    star_adjacent = Some(pos);
                }
            }
        } else {
            if start_digit_pos != 0 {
                if is_part_number {
                    parts.push((&input[start_digit_pos..i], star_adjacent));
                }
                star_adjacent = None;
                start_digit_pos = 0;
                is_part_number = false;
            }
        }
    }
    let numbers = parts
        .iter()
        .flat_map(|(s, _)| str::from_utf8(s))
        .flat_map(|s| s.parse::<usize>());

    println!("parts numbers = {}", numbers.sum::<usize>());

    let star_adjacent_parts: Vec<(usize, &[u8])> = parts
        .iter()
        .filter(|&(_, p)| p.is_some())
        .map(|&(s, p)| (p.unwrap(), s))
        .collect();

    let mut gears: HashMap<usize, Vec<&[u8]>> = HashMap::new();
    for (pos, slice) in star_adjacent_parts {
        let vec = gears.entry(pos).or_insert_with(Vec::new);
        vec.push(slice);
    }
    let gears = gears.iter().filter(|&(_, v)| v.len() == 2).map(|(_, v)| {
        let s1 = str::from_utf8(v[0]);
        let s2 = str::from_utf8(v[1]);
        let s1 = s1.unwrap().parse::<usize>();
        let s2 = s2.unwrap().parse::<usize>();
        s1.unwrap() * s2.unwrap()
    });

    println!("Gear ratios = {}", gears.sum::<usize>());

    Ok(())
}

fn valid_symbol(s: u8) -> bool {
    const SKIP_SYMBOL: u8 = b'.';
    !s.is_ascii_digit() && s != SKIP_SYMBOL
}

fn adjacent_to_symbol(data: &[u8], width: usize, pos: usize) -> Option<usize> {
    // west
    if pos > 0 && valid_symbol(data[pos - 1]) {
        return Some(pos - 1);
    }

    // east
    if pos < data.len() - 1 && valid_symbol(data[pos + 1]) {
        return Some(pos + 1);
    }

    // north
    if pos > width && valid_symbol(data[pos - width]) {
        return Some(pos - width);
    }

    // south
    if pos + width < data.len() && valid_symbol(data[pos + width]) {
        return Some(pos + width);
    }

    // north-west
    if pos > width - 1 && valid_symbol(data[pos - width - 1]) {
        return Some(pos - width - 1);
    }

    // south-east
    if pos + width + 1 < data.len() && valid_symbol(data[pos + width + 1]) {
        return Some(pos + width + 1);
    }

    // north-east
    if pos > width + 1 && valid_symbol(data[pos - width + 1]) {
        return Some(pos - width + 1);
    }

    // south-west
    if pos + width - 1 < data.len() && valid_symbol(data[pos + width - 1]) {
        return Some(pos + width - 1);
    }

    None
}
