use std::fs;

fn within_range(increasing: bool, diff: i32) -> bool {
    if increasing {
        (1..=3).contains(&diff)
    } else {
        (-3..=-1).contains(&diff)
    }
}

fn is_safe(levels: Vec<i32>) -> bool {
    let mut prev: Option<i32> = None;
    let mut increasing: Option<bool> = None;
    let mut safe = true;
    for level in levels.iter() {
        let cur = *level;
        safe = match prev {
            Some(n) => {
                let diff = cur - n;
                match increasing {
                    Some(_increasing) => within_range(_increasing, diff),
                    None => {
                        if diff == 0 {
                            false
                        } else {
                            increasing = Some(diff > 0);
                            within_range(increasing.unwrap(), diff)
                        }
                    }
                }
            }
            None => true,
        };
        prev = Some(cur);
        if !safe {
            return false;
        }
    }

    safe
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines = input.split('\n').collect::<Vec<&str>>();
    let mut num_safe_reports = 0;
    for line in lines {
        let levels_str = line.split(' ').collect::<Vec<&str>>();
        let levels: Vec<i32> = match levels_str.iter().map(|n| n.parse::<i32>()).collect() {
            Ok(levels) => levels,
            Err(_) => {
                continue;
            }
        };
        num_safe_reports += is_safe(levels) as i32;
    }
    println!("Number of safe reports: {}", num_safe_reports);
}
