use lazy_static::lazy_static;
use regex::Regex;
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin().lock();
    let input: Vec<String> = stdin.lines().map(|l| l.unwrap()).collect();

    let red_cubes = 12;
    let green_cubes = 13;
    let blue_cubes = 14;

    let games: Vec<_> = input.iter().filter_map(|line| parse_game(line)).collect();

    let possible_games = games.iter().filter(|game| {
        let (r, g, b) = game.prime_set();
        r <= red_cubes && g <= green_cubes && b <= blue_cubes
    });

    println!(
        "Possible games = {}",
        possible_games.fold(0, |acc, game| acc + game.id)
    );

    let powers_of_games = games.iter().map(|game| {
        let (r, g, b) = game.prime_set();
        r * g * b
    });

    println!("Powers of games = {}", powers_of_games.sum::<u32>());
}

#[derive(Debug)]
enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32),
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Vec<Cube>>,
}

impl Game {
    fn new(id: u32, sets: Vec<Vec<Cube>>) -> Game {
        Game { id, sets }
    }

    fn prime_set(&self) -> (u32, u32, u32) {
        let (mut r, mut g, mut b) = (0, 0, 0);
        for cube in self.sets.iter().flatten() {
            match cube {
                Cube::Red(n) => {
                    if r < *n {
                        r = *n
                    }
                }
                Cube::Green(n) => {
                    if g < *n {
                        g = *n
                    }
                }
                Cube::Blue(n) => {
                    if b < *n {
                        b = *n
                    }
                }
            }
        }
        (r, g, b)
    }
}

fn parse_game(line: &String) -> Option<Game> {
    let mut game_sets = line.split(":");
    let id = game_sets.next()?;
    let parse_game_id = |line: &str| -> Option<u32> {
        lazy_static! {
            static ref GAME_RE: Regex = Regex::new(r"Game\s+(?<id>\d+)\s*").unwrap();
        }
        if let Some(caps) = GAME_RE.captures(line) {
            return caps["id"].parse().ok();
        };
        None
    };
    let id = parse_game_id(id)?;

    lazy_static! {
        static ref CUBE_RE: Regex = Regex::new(r"\s*(?<num>\d+)\s+(?<color>\w+)").unwrap();
    }
    let parse_cube = |line: &str| -> Option<Cube> {
        if let Some(caps) = CUBE_RE.captures(line) {
            let num = caps["num"].parse().ok()?;
            return match &caps["color"] {
                "red" => Some(Cube::Red(num)),
                "green" => Some(Cube::Green(num)),
                "blue" => Some(Cube::Blue(num)),
                _ => None,
            };
        }
        None
    };
    let parse_set = |line: &str| -> Vec<Cube> { line.split(",").filter_map(parse_cube).collect() };

    let sets = game_sets.next()?;
    let sets: Vec<_> = sets.split(";").map(parse_set).collect();

    Some(Game::new(id, sets))
}
