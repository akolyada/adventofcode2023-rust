use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::io::prelude::*;
use std::{char, io, iter};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

trait InsidePoints {
    fn points(&self, row_len: usize) -> usize;
}

struct Maze {
    graph: HashMap<usize, HashSet<usize>>,
    start_vertex: usize,
    start_directions: HashSet<Direction>,
}

fn main() {
    let stdin = io::stdin().lock();

    let input = stdin
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let row_len = input.first().unwrap().len();
    let maze = Maze::new(&input);
    if let Some(main_loop) = maze.find_loop() {
        let steps = main_loop.len() / 2;
        println!("Loop steps = {steps}");

        let inside_points = main_loop.points(row_len);
        println!("Inside points (shoelace-picks) = {inside_points}");

        let inside_points = inside_points_by_ray_tracing(
            &input,
            &HashSet::from_iter(main_loop),
            maze.get_start_point(),
        );
        println!("Inside points (ray-traing) = {inside_points}");
    } else {
        eprintln!("Loop not found!");
    }
}

fn vertex_to_coordinates(vertex: usize, len: usize) -> (isize, isize) {
    ((vertex / len) as isize, (vertex % len) as isize)
}

fn coordinates_to_vertex(row: usize, col: usize, len: usize) -> usize {
    row * len + col
}

fn connected(
    input: &Vec<Vec<char>>,
    direction: Direction,
    row: usize,
    col: usize,
) -> Option<(usize, Direction)> {
    use Direction::*;
    let row_len = input.first().unwrap().len();
    match direction {
        East => {
            let col = col + 1;
            if col >= input[row].len() {
                None
            } else if directions(input[row][col]).contains(&West) {
                Some((coordinates_to_vertex(row, col, row_len), direction))
            } else {
                None
            }
        }
        West => {
            if col == 0 {
                None
            } else if directions(input[row][col - 1]).contains(&East) {
                Some((coordinates_to_vertex(row, col - 1, row_len), direction))
            } else {
                None
            }
        }
        South => {
            let row = row + 1;
            if row >= input.len() {
                None
            } else if directions(input[row][col]).contains(&North) {
                Some((coordinates_to_vertex(row, col, row_len), direction))
            } else {
                None
            }
        }
        North => {
            if row == 0 {
                None
            } else if directions(input[row - 1][col]).contains(&South) {
                Some((coordinates_to_vertex(row - 1, col, row_len), direction))
            } else {
                None
            }
        }
    }
}

fn directions(tile: char) -> &'static HashSet<Direction> {
    use Direction::*;
    lazy_static! {
        static ref NS: HashSet<Direction> = HashSet::from([North, South]);
        static ref EW: HashSet<Direction> = HashSet::from([East, West]);
        static ref NE: HashSet<Direction> = HashSet::from([North, East]);
        static ref NW: HashSet<Direction> = HashSet::from([North, West]);
        static ref SE: HashSet<Direction> = HashSet::from([South, East]);
        static ref SW: HashSet<Direction> = HashSet::from([South, West]);
        static ref ALL: HashSet<Direction> = HashSet::from([South, North, East, West]);
        static ref EMPTY: HashSet<Direction> = HashSet::new();
    }
    match tile {
        '|' => &NS,
        '-' => &EW,
        'L' => &NE,
        'J' => &NW,
        '7' => &SW,
        'F' => &SE,
        'S' => &ALL,
        _ => &EMPTY,
    }
}

fn directions_to_tile(directions: &HashSet<Direction>) -> char {
    use Direction::*;
    if directions.contains(&North) && directions.contains(&South) {
        '|'
    } else if directions.contains(&East) && directions.contains(&West) {
        '-'
    } else if directions.contains(&North) && directions.contains(&East) {
        'L'
    } else if directions.contains(&North) && directions.contains(&West) {
        'J'
    } else if directions.contains(&South) && directions.contains(&West) {
        '7'
    } else if directions.contains(&South) && directions.contains(&East) {
        'F'
    } else {
        panic!("Unknown tile")
    }
}

impl Maze {
    fn new(input: &Vec<Vec<char>>) -> Self {
        let row_len = input.first().unwrap().len();
        let mut graph: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut start_vertex = 0;
        let mut start_directions = HashSet::new();
        for row in 0..input.len() {
            for col in 0..row_len {
                let tile = input[row][col];
                let vertex = coordinates_to_vertex(row, col, row_len);
                for (conn_vertex, conn_direction) in directions(tile)
                    .iter()
                    .filter_map(|&direction| connected(&input, direction, row, col))
                {
                    let list = graph.entry(vertex).or_insert_with(HashSet::new);
                    list.insert(conn_vertex);
                    if tile == 'S' {
                        start_vertex = vertex;
                        start_directions.insert(conn_direction);
                    }
                }
            }
        }
        Maze {
            graph,
            start_vertex,
            start_directions,
        }
    }

    fn find_loop(&self) -> Option<Vec<usize>> {
        let mut visited: HashSet<&usize> = HashSet::new();
        let mut stack: Vec<&usize> = Vec::new();
        let mut result = Vec::new();

        visited.insert(&self.start_vertex);
        result.push(self.start_vertex);

        let mut adjacent_vertices = self.graph.get(&self.start_vertex)?.iter();
        let from_vertex = adjacent_vertices.next()?;
        let to_vertex = adjacent_vertices.next()?;

        stack.push(&from_vertex);

        while let Some(current_vertex) = stack.pop() {
            visited.insert(current_vertex);
            result.push(*current_vertex);

            if *current_vertex == *to_vertex {
                result.push(self.start_vertex);
                return Some(result);
            }

            for v in self.graph.get(&current_vertex)?.iter() {
                if !visited.contains(v) {
                    stack.push(v);
                }
            }
        }

        None
    }

    fn get_start_point(&self) -> (usize, char) {
        (
            self.start_vertex,
            directions_to_tile(&self.start_directions),
        )
    }
}

impl InsidePoints for Vec<usize> {
    // Shoelace formula + Pick's theorem
    fn points(&self, row_len: usize) -> usize {
        let area = (self.windows(2).fold(0, |acc, v| {
            let (x1, y1) = vertex_to_coordinates(v[0], row_len);
            let (x2, y2) = vertex_to_coordinates(v[1], row_len);
            acc + (x1 * y2) - (x2 * y1)
        }) / 2)
            .abs() as usize;
        area + 1 - (self.len() / 2)
    }
}

fn inside_points_by_ray_tracing(
    input: &Vec<Vec<char>>,
    loop_vertices: &HashSet<usize>,
    start_vertice: (usize, char),
) -> usize {
    let row_len = input.first().unwrap().len();
    let mut result = 0;
    for row in 0..input.len() {
        for col in 0..row_len {
            if !loop_vertices.contains(&coordinates_to_vertex(row, col, row_len)) {
                let rays = [
                    // East
                    iter::repeat(row).zip(col + 1..row_len).collect::<Vec<_>>(),
                    // West
                    iter::repeat(row).zip(0..col).collect::<Vec<_>>(),
                    // South
                    (row + 1..input.len())
                        .zip(iter::repeat(col))
                        .collect::<Vec<_>>(),
                    // North
                    (0..row).zip(iter::repeat(col)).collect::<Vec<_>>(),
                ];

                for ray in rays {
                    let mut walls = 0;
                    let mut corner = None;
                    for (row, col) in ray {
                        let mut tile = input[row][col];
                        let vertex = coordinates_to_vertex(row, col, row_len);
                        if loop_vertices.contains(&vertex) {
                            if vertex == start_vertice.0 {
                                tile = start_vertice.1;
                            }
                            match tile {
                                '-' | '|' => {
                                    if corner.is_none() {
                                        walls += 1
                                    }
                                }
                                'L' | '7' | 'F' | 'J' => {
                                    if let Some(c) = corner.take() {
                                        match (c, tile) {
                                            ('F', 'J') | ('L', '7') => walls += 1,
                                            ('J', 'F') | ('7', 'L') => walls += 1,
                                            _ => (),
                                        }
                                    } else {
                                        corner = Some(tile);
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                    if walls != 0 && walls % 2 != 0 {
                        result += 1;
                        break;
                    }
                }
            }
        }
    }
    result
}
