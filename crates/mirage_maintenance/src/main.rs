use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin().lock();

    let report = stdin
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| {
            l.split_whitespace()
                .filter_map(|str| str.parse::<isize>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    let mut extrapolations1 = 0;
    let mut extrapolations2 = 0;
    for history in report {
        let mut diff: Vec<Vec<isize>> = Vec::new();
        diff.push(history);
        loop {
            let seq = diff
                .last()
                .unwrap()
                .windows(2)
                .map(|slice| slice[1].wrapping_sub(slice[0]))
                .collect::<Vec<_>>();
            let finish = seq.iter().all(|&x| x == 0);
            diff.push(seq);
            if finish {
                break;
            }
        }
        let extra1 = diff
            .iter()
            .rev()
            .fold(0, |acc, item| acc + item.last().unwrap());
        let extra2 = diff
            .iter()
            .map(|item| item.first().unwrap())
            .rev()
            .fold(0, |acc, item| item - acc);

        extrapolations1 += extra1;
        extrapolations2 += extra2;
    }
    println!("Extrapolations sum1 = {}", extrapolations1);
    println!("Extrapolations sum2 = {}", extrapolations2);
}
