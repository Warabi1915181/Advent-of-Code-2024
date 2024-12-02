use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in lines {
        let l = line.split("   ").collect::<Vec<&str>>();
        if (l.len() != 2) {
            continue;
        }

        let a = l[0].parse::<i32>().expect("Should be an integer");
        let b = l[1].parse::<i32>().expect("Should be an integer");
        left.push(a);
        right.push(b)
    }

    // sort each column
    left.sort_unstable();
    right.sort_unstable();

    let mut total = 0;

    // compare each pair
    for index in 0..left.len() {
        let dist = left[index] - right[index];
        total += if dist > 0 { dist } else { -dist }
    }
    println!("Total distance: {}\n", total)
}
