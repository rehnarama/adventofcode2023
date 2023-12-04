use std::{
    cell::Cell,
    collections::{HashMap, HashSet},
};

fn main() {
    let input = include_str!("./input.txt");

    let mut extra_tickets: Cell<HashMap<usize, u32>> = Cell::new(HashMap::new());

    let total_scratchcards: u32 = input
        .lines()
        .enumerate()
        .map(|(card_number, line)| {
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
            let n_tickets = *extra_tickets.get_mut().entry(card_number).or_insert(1u32);

            for i in 1..=n_winning {
                *extra_tickets
                    .get_mut()
                    .entry(card_number + i as usize)
                    .or_insert(1) += n_tickets;
            }

            n_tickets
        })
        .sum();

    dbg!(total_scratchcards);
}
