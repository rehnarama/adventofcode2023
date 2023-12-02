use regex::{Captures, Regex};

fn to_digit(s: &str) -> u32 {
    match s {
        "zero" => 0,
        "0" => 0,
        "one" => 1,
        "1" => 1,
        "two" => 2,
        "2" => 2,
        "three" => 3,
        "3" => 3,
        "four" => 4,
        "4" => 4,
        "five" => 5,
        "5" => 5,
        "six" => 6,
        "6" => 6,
        "seven" => 7,
        "7" => 7,
        "eight" => 8,
        "8" => 8,
        "nine" => 9,
        "9" => 9,
        _ => panic!("Not a digit"),
    }
}

fn find_all_matches<'a>(s: &'a str, patterns: Vec<&'a str>) -> Vec<&'a str> {
    let mut matches: Vec<&str> = vec![];

    for i in 1..=s.len() {
        for &pattern in &patterns {
            if pattern.len() > i {
                continue;
            }

            if &s[(i - pattern.len())..i] == pattern {
                matches.push(pattern)
            }
        }
    }

    matches
}

fn main() {
    let input = include_str!("./input.txt");

    let sum: u32 = input
        .lines()
        .map(|line| {
            let captures = find_all_matches(
                line,
                vec![
                    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three",
                    "four", "five", "six", "seven", "eight", "nine", "zero",
                ],
            );
            let &first = captures.first().unwrap();
            let &last = captures.last().unwrap();

            let result = to_digit(first).to_string() + to_digit(last).to_string().as_ref();
            result.parse::<u32>().unwrap()
        })
        .sum();

    dbg!(&sum);
}
