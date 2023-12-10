fn predict(seq: &Vec<i64>) -> i64 {
    let diff = seq
        .iter()
        .zip(seq.iter().skip(1))
        .map(|(&a, &b)| b - a)
        .collect::<Vec<i64>>();

    match diff.iter().all(|&n| n == 0) {
        true => *seq.last().unwrap(),
        false => *seq.last().unwrap() + predict(&diff),
    }
}

fn main() {
    let input = include_str!("./input.txt");

    let sum: i64 = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|digit| digit.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|seq| predict(&seq))
        .sum();

    dbg!(sum);
}
