use std::ops::Range;

fn main() {
    let mut input = include_str!("./input.txt").split("\r\n\r\n");

    let seeds = input
        .next()
        .map(|input| {
            let first_colon = input.find(":").unwrap();
            input[(first_colon + 1)..]
                .trim()
                .split(" ")
                .map(|n| n.trim().parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .unwrap();
    let maps = input
        .map(|map| {
            map.lines()
                .skip(1)
                .map(|line| {
                    let mut split = line.split(" ");
                    let destination_start = split.next().unwrap().parse::<u64>().unwrap();
                    let source_start = split.next().unwrap().parse::<u64>().unwrap();
                    let length = split.next().unwrap().parse::<u64>().unwrap();
                    (
                        source_start..(source_start + length),
                        destination_start..(destination_start + length),
                    )
                })
                .collect::<Vec<(Range<u64>, Range<u64>)>>()
        })
        .collect::<Vec<Vec<(Range<u64>, Range<u64>)>>>();

    let min = seeds
        .iter()
        .map(|seed| {
            maps.iter().fold(*seed, |n, map| {
                let matching_range = map.iter().find(|range| range.0.contains(&n));
                match matching_range {
                    None => n,
                    Some(range) => range.1.start + n - range.0.start,
                }
            })
        })
        .min();

    dbg!(min);
}
