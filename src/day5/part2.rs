use std::{collections::HashSet, ops::Range};

fn main() {
    let mut input = include_str!("./input.txt").split("\r\n\r\n");

    let seeds = input
        .next()
        .map(|input| {
            let first_colon = input.find(":").unwrap();
            let split = input[(first_colon + 1)..]
                .trim()
                .split(" ")
                .collect::<Vec<&str>>();

            let ranges = split
                .chunks(2)
                .map(|chunk| {
                    let start = chunk[0].trim().parse::<u64>().unwrap();
                    let length = chunk[1].trim().parse::<u64>().unwrap();
                    start..(start + length)
                })
                .collect::<Vec<Range<u64>>>();

            ranges
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

    let best_first = maps
        .get(0)
        .unwrap()
        .iter()
        .fold((u64::MAX, None), |min, range| {
            let lowest_score = range
                .to_owned()
                .0
                // Monte carlo yay
                .step_by(1000)
                .map(|seed| {
                    maps.iter().fold(seed, |n, map| {
                        let matching_range = map.iter().find(|range| range.0.contains(&n));
                        match matching_range {
                            None => n,
                            Some(range) => range.1.start + n - range.0.start,
                        }
                    })
                })
                .min()
                .unwrap();

            if min.0 > lowest_score {
                (lowest_score, Some(range))
            } else {
                min
            }
        });
    let best_first = best_first.1.unwrap();

    let min = seeds
        .iter()
        .flat_map(|seed_range| {
            let is_overlapping =
                best_first.1.contains(&seed_range.start) || best_first.1.contains(&seed_range.end);

            let range = if is_overlapping {
                let start = if seed_range.start < best_first.1.start {
                    best_first.1.start
                } else {
                    seed_range.start
                };
                let end = if seed_range.end > best_first.1.end {
                    best_first.1.end
                } else {
                    seed_range.end
                };
                start..end
            } else {
                0..0
            };

            let mapping = range.to_owned().map(|seed| {
                maps.iter().fold(seed, |n, map| {
                    let matching_range = map.iter().find(|range| range.0.contains(&n));
                    match matching_range {
                        None => n,
                        Some(range) => range.1.start + n - range.0.start,
                    }
                })
            });

            mapping
        })
        .min();

    dbg!(min);
}
