use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");

    let total_score: u32 = input
        .lines()
        .map(|line| {
            let first_colon = line.find(":").unwrap();
            let mut split = line[(first_colon + 1)..].split("|");
            let numbers = split
                .next()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<HashSet<u32>>();
            let winning = split
                .next()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<HashSet<u32>>();

            let n_winning = numbers.iter().fold(0u32, |acc, number| {
                acc + if winning.contains(number) { 1 } else { 0 }
            });

            let score = if n_winning > 0 {
                1 << (n_winning - 1)
            } else {
                0
            };

            score
        })
        .sum();

    dbg!(total_score);
}
