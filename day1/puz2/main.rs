use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut count = HashMap::new();
    for line in lines {
        let l = line.split("   ").collect::<Vec<&str>>();
        if (l.len() != 2) {
            continue;
        }

        let a = l[0].parse::<i32>().expect("Should be an integer");
        let b = l[1].parse::<i32>().expect("Should be an integer");
        left.push(a);
        right.push(b);
        match (count.get(&b)) {
            Some(n) => count.insert(b, n + 1),
            None => count.insert(b, 1),
        };
    }
    let mut sum = 0;
    for index in 0..left.len() {
        sum += match (count.get(&left[index])) {
            Some(n) => n,
            None => &0,
        } * left[index];
    }
    println!("Similarity score: {}\n", sum);
}
