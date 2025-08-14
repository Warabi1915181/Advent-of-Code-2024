use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(Clone, Hash, Eq, PartialEq, Copy)]
enum Orientation {
    North,
    South,
    East,
    West,
}

impl Orientation {
    fn turn_90deg(&self) -> Orientation {
        match self {
            Orientation::North => Orientation::East,
            Orientation::East => Orientation::South,
            Orientation::South => Orientation::West,
            Orientation::West => Orientation::North,
        }
    }
}

struct Guard {
    position: Coordinate,
    orientation: Orientation,
}

impl Guard {
    fn move_forward(&mut self, obstructions: &HashSet<Coordinate>, nx: i32, ny: i32) -> bool {
        let next_position = match self.orientation {
            Orientation::North => Coordinate {
                x: self.position.x,
                y: self.position.y - 1,
            },
            Orientation::South => Coordinate {
                x: self.position.x,
                y: self.position.y + 1,
            },
            Orientation::East => Coordinate {
                x: self.position.x + 1,
                y: self.position.y,
            },
            Orientation::West => Coordinate {
                x: self.position.x - 1,
                y: self.position.y,
            },
        };

        if !obstructions.contains(&next_position) && within_bounds(&next_position, nx, ny) {
            self.position = next_position;
            true
        } else if obstructions.contains(&next_position) {
            self.orientation = self.orientation.turn_90deg();
            true
        } else {
            false
        }
    }
    fn is_looping(&self, trail: &HashMap<Coordinate, HashMap<Orientation, bool>>) -> bool {
        if let Some(orientations) = trail.get(&self.position) {
            orientations.contains_key(&self.orientation)
        } else {
            false
        }
    }
}

fn within_bounds(coord: &Coordinate, nx: i32, ny: i32) -> bool {
    coord.x >= 0 && coord.x < nx && coord.y >= 0 && coord.y < ny
}

fn get_obstructions(lines: &[&str]) -> HashSet<Coordinate> {
    let mut obstructions = HashSet::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                obstructions.insert(Coordinate {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }
    obstructions
}

fn get_guard_start(lines: &[&str]) -> Guard {
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '^' | 'v' | '<' | '>' => {
                    let orientation = match c {
                        '^' => Orientation::North,
                        'v' => Orientation::South,
                        '<' => Orientation::West,
                        '>' => Orientation::East,
                        _ => unreachable!(),
                    };
                    let position = Coordinate {
                        x: x as i32,
                        y: y as i32,
                    };
                    return Guard {
                        position,
                        orientation,
                    };
                }
                _ => continue,
            }
        }
    }
    panic!("Guard starting position not found!");
}

fn main() {
    let fs = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let map: Vec<&str> = fs.lines().collect();

    let mut obstructions: HashSet<Coordinate> = get_obstructions(&map);
    let mut guard: Guard = get_guard_start(&map);
    let start_position = guard.position;
    let start_orientation = guard.orientation;
    let mut trail: HashMap<Coordinate, HashMap<Orientation, bool>> = HashMap::new();
    let nx: i32 = map[0].len() as i32;
    let ny: i32 = map.len() as i32;

    let orientations: HashMap<Orientation, bool> =
        [(guard.orientation, true)].iter().cloned().collect();
    trail.insert(guard.position, orientations);

    while guard.move_forward(&obstructions, nx, ny) {
        // first check if the guard is now stepping on a previously visited position in the same
        // direction
        if guard.is_looping(&trail) {
            println!("Guard is looping");
            break;
        }
        match trail.get_mut(&guard.position) {
            Some(orientations) => {
                orientations.entry(guard.orientation).or_insert(true);
            }
            None => {
                let orientations: HashMap<Orientation, bool> =
                    [(guard.orientation, true)].iter().cloned().collect();
                trail.insert(guard.position, orientations);
            }
        }
    }
    println!("Puzzle 1 answer: {}", trail.len());

    let original_trail = trail.clone();
    let visited_positions = original_trail.keys();
    // TODO: find a smarter way to find the possible candidates
    let possible_positions = visited_positions;
    let mut count = 0;

    // Reset states
    guard.position = start_position;
    guard.orientation = start_orientation;
    trail.clear();

    let num_possible_positions = possible_positions.clone().len();
    println!("There are {} visited positions", num_possible_positions);

    for (index, visited) in possible_positions.enumerate() {
        if index % 1000 == 0 || index == num_possible_positions - 1 {
            println!(
                "Processing position {}/{}",
                index + 1,
                num_possible_positions
            );
        }
        if *visited == start_position {
            continue; // Skip the starting position
        }

        let orientations: HashMap<Orientation, bool> =
            [(start_orientation, true)].iter().cloned().collect();
        trail.insert(start_position, orientations);
        obstructions.insert(*visited);
        // println!("Obstruction added at: ({}, {})", visited.x, visited.y);
        while guard.move_forward(&obstructions, nx, ny) {
            // first check if the guard is now stepping on a previously visited position in the same
            // direction

            if guard.is_looping(&trail) {
                count += 1;
                break;
            }
            match trail.get_mut(&guard.position) {
                Some(orientations) => {
                    orientations.entry(guard.orientation).or_insert(true);
                }
                None => {
                    let orientations: HashMap<Orientation, bool> =
                        [(guard.orientation, true)].iter().cloned().collect();
                    trail.insert(guard.position, orientations);
                }
            }
        }
        obstructions.remove(visited);
        trail.clear();
        guard.position = start_position;
        guard.orientation = start_orientation;
    }
    println!("Puzzle 2 answer: {}", count);
}
