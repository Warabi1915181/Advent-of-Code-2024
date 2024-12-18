use regex::Regex;
use std::fs;

fn mul(expr: &str) -> i32 {
    let re = Regex::new(r"(?<arg1>[0-9]*),(?<arg2>[0-9]*)").unwrap();
    let caps = re.captures(expr).unwrap();
    let arg1 = &caps["arg1"].parse::<i32>().unwrap();
    let arg2 = &caps["arg2"].parse::<i32>().unwrap();
    arg1 * arg2
}

fn scan_text(text: &str) -> i32 {
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    re.find_iter(text)
        .map(|m| m.as_str())
        .map(mul)
        .reduce(|acc, cur| acc + cur)
        .unwrap_or(0)
}

fn scan_text_conditional(text: &str) -> i32 {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let mut parse = true;
    let mut sum = 0;
    for m in re.find_iter(text) {
        let expr = m.as_str();
        if expr.eq("do()") | expr.eq("don't()") {
            parse = expr.eq("do()");
            continue;
        } else if parse {
            sum += mul(expr);
        }
    }
    sum
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    // let lines = input.split('\n');
    let res1 = scan_text(input.as_str());
    let res2 = scan_text_conditional(input.as_str());
    println!("Result of puzzle 1: {}", res1);
    println!("Result of puzzle 2: {}", res2);
}
