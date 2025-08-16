use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn find_antinodes_from_two_antennas(
    antenna_1: (i32, i32),
    antenna_2: (i32, i32),
) -> ((i32, i32), (i32, i32)) {
    (
        (2 * antenna_1.0 - antenna_2.0, 2 * antenna_1.1 - antenna_2.1),
        (2 * antenna_2.0 - antenna_1.0, 2 * antenna_2.1 - antenna_1.1),
    )
}

type AntennaMap = HashMap<char, Vec<(i32, i32)>>;

fn get_antennas(lines: Vec<String>) -> AntennaMap {
    let mut antennas: AntennaMap = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        let tokens: Vec<char> = line.chars().collect();
        for (x, token) in tokens.iter().enumerate() {
            if token == &'.' {
                continue;
            }
            if let Some(antenna) = antennas.get_mut(token) {
                antenna.push((x as i32, y as i32));
            } else {
                antennas.insert(*token, vec![(x as i32, y as i32)]);
            }
        }
    }

    antennas
}

fn get_antinode_from_antennas(antennas: &AntennaMap, nx: i32, ny: i32) -> HashSet<(i32, i32)> {
    let mut antinodes: Vec<(i32, i32)> = Vec::new();
    for (_, antinodes_with_same_freq) in antennas.iter() {
        if antinodes_with_same_freq.len() < 2 {
            continue;
        }

        for i in 0..antinodes_with_same_freq.len() {
            for j in (i + 1)..antinodes_with_same_freq.len() {
                let (antinode_1, antinode_2) = find_antinodes_from_two_antennas(
                    antinodes_with_same_freq[i],
                    antinodes_with_same_freq[j],
                );
                if !is_out_of_bounds(antinode_1, nx, ny) {
                    antinodes.push(antinode_1);
                }
                if !is_out_of_bounds(antinode_2, nx, ny) {
                    antinodes.push(antinode_2);
                }
            }
        }
    }

    antinodes.into_iter().collect()
}

fn is_out_of_bounds(point: (i32, i32), nx: i32, ny: i32) -> bool {
    point.0 < 0 || point.1 < 0 || point.0 >= nx || point.1 >= ny
}

fn main() {
    let fs = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines: Vec<String> = fs.lines().map(|line| line.to_string()).collect();

    let nx = lines[0].len() as i32;
    let ny = lines.len() as i32;

    let antennas: AntennaMap = get_antennas(lines);
    let antinodes = get_antinode_from_antennas(&antennas, nx, ny);
    let unique_locations_with_antinode = antinodes.len();
    println!("Puzzle 1 ans: {}", unique_locations_with_antinode);
}
