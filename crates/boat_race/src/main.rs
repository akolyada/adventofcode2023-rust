use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let stdin = io::stdin().lock();
    let lines = stdin.lines().collect::<io::Result<Vec<String>>>()?;

    let input: Vec<Vec<&str>> = lines
        .iter()
        .flat_map(|l| l.split(":").skip(1))
        .map(|s| s.split_whitespace().collect::<Vec<&str>>())
        .collect();

    let mut input1 = input.iter();

    let times = input1.next().unwrap();
    let distances = input1.next().unwrap();

    let input1: Vec<(usize, usize)> = times
        .iter()
        .filter_map(|s| s.parse::<usize>().ok())
        .zip(distances.iter().filter_map(|s| s.parse::<usize>().ok()))
        .collect();

    let win_count: usize = input1
        .iter()
        .map(|(time, distance)| {
            (1..*time)
                .map(|t| travel(t, *time))
                .filter(|d| d > distance)
                .count()
        })
        .product();

    println!("win count 1 = {}", win_count);

    let mut input2 = input.iter();

    let times = input2.next().unwrap();
    let distances = input2.next().unwrap();

    let time: usize = times.join("").parse().unwrap();
    let distance: usize = distances.join("").parse().unwrap();

    let win_count: usize = (1..time)
        .map(|t| travel(t, time))
        .filter(|d| *d > distance)
        .count();
    println!("win count 2 = {}", win_count);

    Ok(())
}

fn travel(hold_time: usize, total_time: usize) -> usize {
    const SPEED_INCREASE_TEMPO: usize = 1; // 1 mm/ms
    let speed = hold_time * SPEED_INCREASE_TEMPO;
    (total_time - hold_time) * speed
}
