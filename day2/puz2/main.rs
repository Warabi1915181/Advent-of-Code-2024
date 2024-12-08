use std::fs;

fn within_range(increasing: bool, diff: i32) -> bool {
    if increasing {
        (1..=3).contains(&diff)
    } else {
        (-3..=-1).contains(&diff)
    }
}

fn is_safe(levels: Vec<i32>) -> bool {
    fn _is_safe(levels: Vec<i32>, increasing: bool) -> bool {
        for (i, level) in levels.iter().enumerate() {
            if i == 0 {
                continue;
            };
            let diff = *level - levels[i - 1];
            let safe = within_range(increasing, diff);
            if !safe {
                return false;
            }
        }
        true
    }

    for i in 0..levels.len() {
        let increasing = _is_safe(
            levels
                .clone()
                .into_iter()
                .enumerate()
                .filter(|(j, _)| i != *j)
                .map(|(_, level)| level)
                .collect::<Vec<i32>>(),
            true,
        );

        let decreasing = _is_safe(
            levels
                .clone()
                .into_iter()
                .enumerate()
                .filter(|(j, _)| i != *j)
                .map(|(_, level)| level)
                .collect::<Vec<i32>>(),
            false,
        );

        if increasing || decreasing {
            return true;
        }
    }

    false
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines = input.split('\n').collect::<Vec<&str>>();
    let mut num_safe_reports = 0;
    for line in lines {
        let levels_str = line.split_whitespace().collect::<Vec<&str>>();
        let levels: Vec<i32> = match levels_str.iter().map(|n| n.parse::<i32>()).collect() {
            Ok(levels) => levels,
            Err(_) => {
                continue;
            }
        };
        if levels.is_empty() {
            continue;
        }
        let safe = is_safe(levels);

        num_safe_reports += safe as i32;
    }
    println!("Number of safe reports: {}", num_safe_reports);
}
