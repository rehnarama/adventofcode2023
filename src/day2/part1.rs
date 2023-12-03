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

    let max_red: usize = 12;
    let max_green: usize = 13;
    let max_blue: usize = 14;

    let sum = input
        .lines()
        .map(|line| {
            let first_colon = line.find(":").unwrap();
            let game = &line[(first_colon + 1)..];
            let rounds = game.split(";");
            let cubes_per_round: Vec<Vec<Cube>> = rounds
                .map(|round| {
                    let cubes: Vec<Cube> = round.split(",").map(|cube| cube.into()).collect();

                    cubes
                })
                .collect();
            let is_possible_per_round: Vec<bool> = cubes_per_round
                .iter()
                .map(|cubes| {
                    cubes.iter().all(|cube| match cube {
                        Cube::Red(amount) => max_red >= *amount,
                        Cube::Green(amount) => max_green >= *amount,
                        Cube::Blue(amount) => max_blue >= *amount,
                    })
                })
                .collect();

            let is_game_possible = is_possible_per_round.iter().all(|possible| *possible);

            is_game_possible
        })
        .enumerate()
        .fold(
            0,
            |sum, (game_id, is_possible)| {
                if is_possible {
                    sum + game_id + 1
                } else {
                    sum
                }
            },
        );

    dbg!(sum);
}
