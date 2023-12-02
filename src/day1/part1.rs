use regex::{Captures, Regex};

fn main() {
    let input = include_str!("./input.txt");

    let sum: u32 = input
        .lines()
        .map(|line| {
            let digit_regex = Regex::new(r"\d").unwrap();
            let captures: Vec<Captures> = digit_regex.captures_iter(line).collect();
            let first = captures.first().unwrap().get(0).unwrap();
            let last = captures.last().unwrap().get(0).unwrap();

            let result = first.as_str().to_owned() + last.as_str();
            result.parse::<u32>().unwrap()
        })
        .sum();

    dbg!(&sum);
}
