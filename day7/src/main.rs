use regex::Regex;
use std::fs;

fn dfs(numbers: &Vec<i64>, cur: usize, accumulator: i64, target: i64) -> bool {
    if cur >= numbers.len() - 1 {
        return accumulator == target;
    }

    if dfs(numbers, cur + 1, accumulator + numbers[cur + 1], target) {
        // If we found a valid path, we can return true immediately
        return true;
    }
    if dfs(numbers, cur + 1, accumulator * numbers[cur + 1], target) {
        // If we found a valid path, we can return true immediately
        return true;
    }
    false
}

fn solve_line(line: &str) -> i64 {
    let re = Regex::new(r"([0-9]*):([0-9|\ ]*)").unwrap();
    let res = re.captures(line).unwrap();
    let target: i64 = res.get(1).unwrap().as_str().parse().unwrap();
    let numbers: Vec<i64> = res
        .get(2)
        .unwrap()
        .as_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    if dfs(&numbers, 0, numbers[0], target) {
        target
    } else {
        0
    }
}

fn main() {
    let fs = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines: Vec<&str> = fs.lines().collect();
    let mut total = 0;
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let result = solve_line(line);
        total += result;
    }

    println!("Puzzle 1 answer: {}", total);
}
