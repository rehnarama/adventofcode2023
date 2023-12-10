use core::panic;
use std::collections::{HashMap, HashSet, VecDeque};

fn get_next_position(pipes: &HashMap<(i32, i32), char>, pos: &(i32, i32)) -> [(i32, i32); 2] {
    let pipe = pipes.get(pos).unwrap();

    match *pipe {
        '|' => [(pos.0, pos.1 - 1), ((pos.0, pos.1 + 1))],
        '-' => [(pos.0 - 1, pos.1), ((pos.0 + 1, pos.1))],
        'F' => [(pos.0 + 1, pos.1), ((pos.0, pos.1 + 1))],
        '7' => [(pos.0 - 1, pos.1), ((pos.0, pos.1 + 1))],
        'L' => [(pos.0 + 1, pos.1), ((pos.0, pos.1 - 1))],
        'J' => [(pos.0 - 1, pos.1), ((pos.0, pos.1 - 1))],
        _ => panic!("Not a valid pipe"),
    }
}

fn main() {
    let input = include_str!("./input.txt");

    let mut pipes: HashMap<(i32, i32), char> = HashMap::new();
    let mut start: (i32, i32) = (0, 0);

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == 'S' {
                start = (x as i32, y as i32);
            } else if c != '.' {
                pipes.insert((x as i32, y as i32), c);
            }
        });
    });

    let has_north = pipes.contains_key(&(start.0, start.1 - 1));
    let has_south = pipes.contains_key(&(start.0, start.1 + 1));
    let has_west = pipes.contains_key(&(start.0 - 1, start.1));
    let has_east = pipes.contains_key(&(start.0 + 1, start.1));

    let start_pipe = if has_north && has_south {
        '|'
    } else if has_west && has_east {
        '-'
    } else if has_south && has_east {
        'F'
    } else if has_south && has_west {
        '7'
    } else if has_north && has_east {
        'L'
    } else {
        'J'
    };
    pipes.insert(start, start_pipe);

    let mut next: VecDeque<(i32, i32)> = VecDeque::new();
    next.push_back(start);
    let mut distances: HashMap<(i32, i32), u32> = HashMap::new();

    while !next.is_empty() {
        let pos = next.pop_front().unwrap();

        for next_position in get_next_position(&pipes, &pos) {
            match distances.get(&next_position) {
                Some(&value) => {
                    let min_distance = distances.entry(pos).or_insert(value + 1);

                    *min_distance = min_distance.to_owned().max(value + 1);
                }
                None => {
                    distances.entry(pos).or_insert(0);
                    next.push_back(next_position);
                }
            }
        }
    }

    let max_distance = distances
        .values()
        .max();

    dbg!(max_distance);
}
