use core::panic;
use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

fn get_box(c: &char, start_pipe: char) -> char {
    match *c {
        '|' => '┃',
        '-' => '━',
        'F' => '┏',
        '7' => '┓',
        'L' => '┗',
        'J' => '┛',
        'S' => get_box(&start_pipe, start_pipe),
        _ => panic!("Not a valid pipe: {}", c),
    }
}

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

fn is_connected(pipes: &HashMap<(i32, i32), char>, from: &(i32, i32), to: &(i32, i32)) -> bool {
    if pipes.contains_key(from) {
        get_next_position(pipes, from).iter().any(|pos| pos == to)
    } else {
        false
    }
}

fn main() {
    let input = include_str!("./input.txt");

    let mut pipes: HashMap<(i32, i32), char> = HashMap::new();
    let mut ground: HashSet<(i32, i32)> = HashSet::new();
    let mut start: (i32, i32) = (0, 0);

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == 'S' {
                start = (x as i32, y as i32);
            } else if c != '.' {
                pipes.insert((x as i32, y as i32), c);
            } else if c == '.' {
                ground.insert((x as i32, y as i32));
            }
        });
    });

    let has_north = is_connected(&pipes, &(start.0, start.1 - 1), &start);
    let has_south = is_connected(&pipes, &(start.0, start.1 + 1), &start);
    let has_west = is_connected(&pipes, &(start.0 - 1, start.1), &start);
    let has_east = is_connected(&pipes, &(start.0 + 1, start.1), &start);

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
    let mut loop_points: HashSet<(i32, i32)> = HashSet::new();

    while !next.is_empty() {
        let pos = next.pop_front().unwrap();

        for next_position in get_next_position(&pipes, &pos) {
            if !loop_points.contains(&next_position) {
                loop_points.insert(next_position);
                next.push_back(next_position);
            }
        }
    }

    let mut intersections: HashMap<(i32, i32), u32> = HashMap::new();
    for row in 0..input.lines().count() {
        for col in 0..input.lines().next().unwrap().len() {
            let point = (col as i32, row as i32);
            if loop_points.contains(&point) {
                continue;
            }

            let delta_north = if row == 0 { 1 } else { -1 };

            let mut hits = 0u32;
            let y = point.1;
            for x in 0..=point.0 {
                let is_loop_point = loop_points.contains(&(x, y));
                let has_north =
                    is_loop_point && is_connected(&pipes, &(x, y), &(x, y + delta_north));

                if has_north {
                    hits += 1;
                }
            }
            intersections.insert(point, hits);
        }
    }

    let n_enclosed: u32 = intersections
        .values()
        .filter(|&intersections| intersections % 2 == 1)
        .map(|_| 1)
        .sum();

    dbg!(n_enclosed);
}
