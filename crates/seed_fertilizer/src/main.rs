use rayon::prelude::*;
use std::io::prelude::*;
use std::{io, usize};

fn main() {
    let stdin = io::stdin().lock();

    let mut lines = stdin.lines().filter_map(|l| l.ok());

    let seeds: Vec<usize> = lines
        .next()
        .unwrap()
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    let mut seed_to_soil: Vec<(usize, usize, usize)> = Vec::new();
    let mut soil_to_fertilizer: Vec<(usize, usize, usize)> = Vec::new();
    let mut fertilizer_to_water: Vec<(usize, usize, usize)> = Vec::new();
    let mut water_to_ligth: Vec<(usize, usize, usize)> = Vec::new();
    let mut light_to_temperature: Vec<(usize, usize, usize)> = Vec::new();
    let mut temperature_to_humidity: Vec<(usize, usize, usize)> = Vec::new();
    let mut humidity_to_location: Vec<(usize, usize, usize)> = Vec::new();

    let mut map: &mut Vec<(usize, usize, usize)> = &mut seed_to_soil;
    while let Some(line) = lines.next() {
        if line.is_empty() {
            if let Some(state_line) = lines.next() {
                match state_line.as_str() {
                    "seed-to-soil map:" => map = &mut seed_to_soil,
                    "soil-to-fertilizer map:" => map = &mut soil_to_fertilizer,
                    "fertilizer-to-water map:" => map = &mut fertilizer_to_water,
                    "water-to-light map:" => map = &mut water_to_ligth,
                    "light-to-temperature map:" => map = &mut light_to_temperature,
                    "temperature-to-humidity map:" => map = &mut temperature_to_humidity,
                    "humidity-to-location map:" => map = &mut humidity_to_location,
                    s => eprintln!("Unexpected map: {s}"),
                }
            }
            continue;
        }

        let range: Vec<usize> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();

        map.push((range[0], range[1], range[2]));
    }

    let lowest_location = seeds
        .iter()
        .map(|&s| convert(s, &seed_to_soil))
        .map(|s| convert(s, &soil_to_fertilizer))
        .map(|f| convert(f, &fertilizer_to_water))
        .map(|w| convert(w, &water_to_ligth))
        .map(|l| convert(l, &light_to_temperature))
        .map(|t| convert(t, &temperature_to_humidity))
        .map(|h| convert(h, &humidity_to_location))
        .min();

    println!("lowest location = {}", lowest_location.unwrap());

    assert!(seeds.len() % 2 == 0);

    let seeds: Vec<usize> = seeds
        .chunks(2)
        .flat_map(|seeds| (seeds[0]..seeds[0] + seeds[1]))
        .collect();

    let lowest_location2 = seeds
        .par_iter()
        .map(|&s| convert(s, &seed_to_soil))
        .map(|s| convert(s, &soil_to_fertilizer))
        .map(|f| convert(f, &fertilizer_to_water))
        .map(|w| convert(w, &water_to_ligth))
        .map(|l| convert(l, &light_to_temperature))
        .map(|t| convert(t, &temperature_to_humidity))
        .map(|h| convert(h, &humidity_to_location))
        .min();

    println!("lowest location2 = {}", lowest_location2.unwrap());
}

fn convert(item: usize, ranges: &Vec<(usize, usize, usize)>) -> usize {
    for (dest_start, src_start, len) in ranges {
        if &item >= src_start && item < src_start + len {
            return dest_start + (item - src_start);
        }
    }
    item
}
