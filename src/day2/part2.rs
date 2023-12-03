use std::cmp::min;

enum Cube {
    Red(usize),
    Green(usize),
    Blue(usize),
}

impl From<&str> for Cube {
    fn from(value: &str) -> Self {
        let mut split = value.trim().split(" ");
        let digit = split.next().unwrap();
        let color = split.next().unwrap();

        match color {
            "red" => Cube::Red(digit.parse().unwrap()),
            "green" => Cube::Green(digit.parse().unwrap()),
            "blue" => Cube::Blue(digit.parse().unwrap()),
            _ => panic!("Not a valid color"),
        }
    }
}

fn main() {
    let input = include_str!("./input.txt");

    let sum: usize = input
        .lines()
        .map(|line| {
            let first_colon = line.find(":").unwrap();
            let game = &line[(first_colon + 1)..];
            let rounds = game.split(";");
            let cubes_per_round: Vec<(usize, usize, usize)> = rounds
                .map(|round| {
                    let cubes: Vec<Cube> = round.split(",").map(|cube| cube.into()).collect();

                    cubes.iter().fold((0, 0, 0), |acc, cube| match cube {
                        Cube::Red(amount) => (amount + acc.0, acc.1, acc.2),
                        Cube::Green(amount) => (acc.0, amount + acc.1, acc.2),
                        Cube::Blue(amount) => (acc.0, acc.1, amount + acc.2),
                    })
                })
                .collect();
            let minimum_per_round = cubes_per_round.iter().fold((0, 0, 0), |acc, cubes| {
                (
                    std::cmp::max(acc.0, cubes.0),
                    std::cmp::max(acc.1, cubes.1),
                    std::cmp::max(acc.2, cubes.2),
                )
            });

            let power_of_minimum = minimum_per_round.0 * minimum_per_round.1 * minimum_per_round.2;

            power_of_minimum
        })
        .sum();

    dbg!(sum);
}
