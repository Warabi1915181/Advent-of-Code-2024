use std::fs;

fn is_safe(mut increasing: Option<bool>, prev: Option<i32>, cur: i32) -> (Option<bool>, bool) {
    // return true if the difference is within the accepted boundary
    fn check_diff(increasing: bool, diff: i32) -> bool {
        if increasing {
            (1..=3).contains(&diff)
        } else {
            (-3..=-1).contains(&diff)
        }
    }

    match prev {
        Some(n) => {
            let diff = cur - n;
            match increasing {
                Some(_increasing) => (increasing, check_diff(_increasing, diff)),
                None => {
                    if diff == 0 {
                        (increasing, false)
                    } else {
                        increasing = Some(diff > 0);
                        (increasing, check_diff(increasing.unwrap(), diff))
                    }
                }
            }
        }
        None => (increasing, true),
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines = input.split('\n').collect::<Vec<&str>>();
    let mut num_safe_reports = 0;
    for line in lines {
        let _levels = line.split(' ').collect::<Vec<&str>>();
        let levels_num = _levels.iter().map(|n| n.parse::<i32>());
        let levels: Vec<i32> = match levels_num.collect() {
            Ok(levels) => levels,
            Err(_) => {
                continue;
            }
        };

        let mut prev: Option<i32> = None;
        let mut increasing: Option<bool> = None;
        let mut safe = true;
        for level in levels.iter() {
            let cur = *level;
            let _safe;
            (increasing, _safe) = is_safe(increasing, prev, cur);
            safe = safe && _safe;
            prev = Some(cur);
        }
        num_safe_reports += safe as i32;
    }
    println!("Number of safe reports: {}", num_safe_reports);
}
