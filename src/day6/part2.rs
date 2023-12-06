use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");

    let mut lines = input.lines();
    let times = lines
        .next()
        .map(|line| {
            let first_colon = line.find(":").unwrap();
            let numbers = (&line[(first_colon + 1)..])
                .trim()
                .split_whitespace()
                .join("")
                .parse::<u64>()
                .unwrap();

            numbers
        })
        .unwrap();

    let distances = lines
        .next()
        .map(|line| {
            let first_colon = line.find(":").unwrap();
            let numbers = (&line[(first_colon + 1)..])
                .trim()
                .split_whitespace()
                .join("")
                .parse::<u64>()
                .unwrap();

            numbers
        })
        .unwrap();

    let times = vec![times];
    let distances = vec![distances];

    let n_winning_times = times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| {
            let time = time as f64;
            let distance = distance as f64 + 1.; // We must be at least 1mm better!
            let first_root = (time / 2.) - (time.powf(2.) / 4. - distance).sqrt();
            let second_root = (time / 2.) + (time.powf(2.) / 4. - distance).sqrt();

            (second_root.floor() - first_root.ceil()) as u64 + 1 // Inclusive range
        })
        .reduce(|a, b| a * b)
        .unwrap();

    dbg!(n_winning_times);
}
