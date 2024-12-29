use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

fn parse_rules(rules: &str) -> HashMap<i32, Vec<i32>> {
    let mut rules_map: HashMap<i32, Vec<i32>> = HashMap::new();
    for rule in rules.split('\n').collect::<Vec<&str>>() {
        let parts = rule.split('|').collect::<Vec<&str>>();
        let key = parts[0].parse::<i32>().unwrap();
        let value = parts[1].parse::<i32>().unwrap();
        match rules_map.get_mut(&key) {
            Some(values) => {
                values.push(value);
            }
            None => {
                rules_map.insert(key, vec![value]);
            }
        }
    }
    rules_map
}

fn is_update_valid(rules: HashMap<i32, Vec<i32>>, update: &str) -> bool {
    let _update = update
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut valid = true;
    for (i, lhs) in _update.iter().enumerate() {
        valid = valid
            && _update.clone()[i + 1..]
                .iter()
                .fold(true, |acc, cur| match rules.get(cur) {
                    Some(rhs) => !rhs.contains(lhs) && acc,
                    None => acc,
                })
    }
    valid
}

fn get_updates(rules: HashMap<i32, Vec<i32>>, updates: Vec<&str>, valid: bool) -> Vec<&str> {
    updates
        .clone()
        .into_iter()
        .filter(|update| {
            if valid {
                is_update_valid(rules.clone(), update)
            } else {
                !is_update_valid(rules.clone(), update)
            }
        })
        .collect()
}

fn get_middle_update<T: AsRef<str>>(update: T) -> i32 {
    let update_i32 = update
        .as_ref()
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mid = update_i32.len() / 2;
    update_i32[mid]
}

fn compare(rules: HashMap<i32, Vec<i32>>, x: &i32, y: &i32) -> Ordering {
    if let Some(rhs) = rules.get(x) {
        if rhs.contains(y) {
            return Ordering::Less;
        }
    }
    if let Some(rhs) = rules.get(y) {
        if rhs.contains(x) {
            return Ordering::Greater;
        }
    }
    Ordering::Equal
}

fn reorder_invalid_updates(rules: HashMap<i32, Vec<i32>>, updates: Vec<&str>) -> Vec<String> {
    updates
        .into_iter()
        .map(|update| {
            let mut update_i32: Vec<i32> = update
                .split(',')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            update_i32.sort_by(|x, y| compare(rules.clone(), x, y));
            update_i32
                .into_iter()
                .map(|x| x.to_string())
                .reduce(|acc, cur| acc + "," + &cur)
                .unwrap()
        })
        .collect::<Vec<String>>()
}

fn main() {
    let fs = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let input: Vec<&str> = fs.split("\n\n").filter(|s| !s.is_empty()).collect();
    let rules = parse_rules(input[0]);
    let updates = input[1]
        .lines()
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();
    let valid_updates = get_updates(rules.clone(), updates.clone(), true);
    let sum_middle: i32 = valid_updates.into_iter().map(get_middle_update).sum();

    let invalid_updates = get_updates(rules.clone(), updates.clone(), false);
    let reordered_invalid_updates = reorder_invalid_updates(rules.clone(), invalid_updates);
    let sum_middle_invalid_reordered: i32 = reordered_invalid_updates
        .into_iter()
        .map(get_middle_update)
        .sum();
    println!("Puzzle 1 result: {}", sum_middle);
    println!("Puzzle 2 result: {}", sum_middle_invalid_reordered);
}
