use core::num;
use std::{
    cell::Cell,
    collections::{HashMap, HashSet},
};

#[derive(Debug)]
struct EnginePart<'a> {
    start_col: usize,
    end_col: usize,
    row: usize,
    line: &'a str,
}

fn main() {
    let input = include_str!("./input.txt");

    let mut engine_parts: Vec<EnginePart> = vec![];
    let mut potential_gears: HashSet<(i32, i32)> = HashSet::new();

    for (row, line) in input.lines().enumerate() {
        // let mut start: Option<usize> = None;
        // let mut end: Option<usize> = None;
        let mut part: Cell<Option<&mut EnginePart>> = Cell::new(None);
        for (col, char) in line.chars().enumerate() {
            if char >= '0' && char <= '9' {
                match part.get_mut() {
                    None => {
                        engine_parts.push(EnginePart {
                            start_col: col,
                            end_col: col,
                            row: row,
                            line,
                        });
                        part = Cell::new(engine_parts.last_mut());
                    }
                    Some(part) => {
                        part.end_col = col;
                    }
                }
            } else if char == '*' {
                potential_gears.insert((col as i32, row as i32));
                part = Cell::new(None);
            } else {
                part = Cell::new(None);
            }
        }
    }

    let number_count: u32 = potential_gears
        .iter()
        .map(|&(x, y)| {
            engine_parts
                .iter()
                .map(move |part| {
                    if x >= (part.start_col as i32 - 1)
                        && x <= (part.end_col as i32 + 1)
                        && 1 >= (y - part.row as i32).abs()
                    {
                        Some(part)
                    } else {
                        None
                    }
                })
                .filter(|opt| opt.is_some())
                .map(|opt| opt.unwrap())
                .collect::<Vec<&EnginePart>>()
        })
        .filter(|parts| parts.len() == 2)
        .flat_map(|parts| {
            parts
                .iter()
                .map(|part| part.line[part.start_col..=part.end_col].parse::<u32>().unwrap())
                .reduce(|a, b| a * b)
        })
        .sum();

    dbg!(number_count);
}
