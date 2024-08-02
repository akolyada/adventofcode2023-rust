use num;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin().lock();

    let mut lines = stdin.lines();
    let instruction = lines.next().unwrap().unwrap();

    let lines: Vec<String> = lines.skip(1).filter_map(|l| l.ok()).collect();

    let mut network: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut keys_step2: Vec<&str> = Vec::new();
    for line in lines.iter() {
        if let Some((key, val)) = line.split_once(" = ") {
            if let Some((lkey, rkey)) = val.split_once(", ") {
                let mut lkey = lkey.chars();
                lkey.next();
                let mut rkey = rkey.chars();
                rkey.next_back();
                network.insert(key, (lkey.as_str(), rkey.as_str()));
            }
            if key.ends_with('A') {
                keys_step2.push(key);
            }
        }
    }

    let steps = find_steps_count("AAA", instruction.as_str(), &network);

    println!("Steps = {}", steps);

    let mut key_steps: Vec<usize> = Vec::new();
    for key in keys_step2.iter() {
        key_steps.push(find_steps_count(key, instruction.as_str(), &network));
    }

    let mut lcm = 1;
    for steps in key_steps {
        lcm = num::integer::lcm(lcm, steps);
    }

    println!("Steps2 = {}", lcm);
}

fn find_steps_count(
    start: &str,
    instruction: &str,
    network: &HashMap<&str, (&str, &str)>,
) -> usize {
    let mut next = start;
    let mut steps = 0;

    'outer: loop {
        for c in instruction.chars() {
            if let Some(direction) = network.get(next) {
                next = if c == 'L' { direction.0 } else { direction.1 };
            }
            steps += 1;
            if next.ends_with('Z') {
                break 'outer;
            }
        }
    }

    steps
}
