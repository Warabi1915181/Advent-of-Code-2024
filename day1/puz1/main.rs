use std::fs;

fn main() {
    println!("Hello world");
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut pair_a = Vec::new();
    let mut pair_b = Vec::new();
    for line in lines {
        let l = line.split("   ").collect::<Vec<&str>>();
        if (l.len() != 2) {
            continue;
        }

        let a = l[0].parse::<i32>();
        let b = l[1].parse::<i32>();
        match a {
            Ok(n) => pair_a.push(n),
            Err(e) => println!("{}", e),
        }
        match b {
            Ok(n) => pair_b.push(n),
            Err(e) => println!("{}", e),
        }
    }

    // sort each column
    pair_a.sort_unstable();
    pair_b.sort_unstable();

    let mut total = 0;

    // compare each pair
    for index in 0..pair_a.len() {
        let dist = pair_a[index] - pair_b[index];
        total += if dist > 0 { dist } else { -dist }
    }
    println!("Total distance: {}\n", total)
}
