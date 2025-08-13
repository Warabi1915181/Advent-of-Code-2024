use std::collections::HashSet;
use std::fs;

#[derive(Eq, Hash, PartialEq, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
}

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
    let fs = fs::read_to_string("example.txt").expect("Should have been able to read the file");
    let map: Vec<&str> = fs.lines().collect();

    let obstructions: HashSet<Coordinate> = get_obstructions(&map);
    let mut guard: Guard = get_guard_start(&map);
    let mut footprint: HashSet<Coordinate> = HashSet::new();
    let nx: i32 = map[0].len() as i32;
    let ny: i32 = map.len() as i32;

    footprint.insert(guard.position.clone());

    let mut iter_stuck = 0;
    let last: Coordinate = guard.position.clone();

    while guard.move_forward(&obstructions, nx, ny) {
        footprint.insert(guard.position.clone());
        // safe guard to prevent infinite loop
        if guard.position == last {
            iter_stuck += 1;
            if iter_stuck > 1000 {
                panic!("Guard is stuck, stopping.");
            }
        } else {
            iter_stuck = 0;
        }
    }
    println!("Answer: {}", footprint.len());
}
