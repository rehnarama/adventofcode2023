use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");

    let mut lines = input.lines();

    let instructions = lines.next().unwrap();

    let mut graph: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in lines.skip(1) {
        let (first, second) = line.split_once(" = ").unwrap();
        let second = &second[1..(second.len() - 1)];
        let (left, right) = second.split_once(", ").unwrap();
        graph.insert(first, (left, right));
    }

    let mut location = "AAA";
    let mut n_steps = 0usize;
    while location != "ZZZ" {
        let instruction = instructions
            .chars()
            .nth(n_steps % instructions.len())
            .unwrap();

        let possible_paths = graph.get(location).unwrap();
        location = match instruction {
            'L' => possible_paths.0,
            'R' => possible_paths.1,
            _ => panic!("Not left or right"),
        };

        n_steps += 1;
    }

    dbg!(n_steps);
}
