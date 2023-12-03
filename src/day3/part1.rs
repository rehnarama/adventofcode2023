use std::{cell::Cell, collections::HashSet};

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
    let mut special_chars: HashSet<(i32, i32)> = HashSet::new();

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
            } else if char != '.' {
                special_chars.insert((col as i32, row as i32));
                part = Cell::new(None);
            } else {
                part = Cell::new(None);
            }
        }
    }

    let valid_engine_parts_sum: i32 = engine_parts
        .iter()
        .filter(|part| {
            let start_col = part.start_col as i32;
            let end_col = part.end_col as i32;
            let row = part.row as i32;
            for x in (start_col - 1)..=(end_col + 1) {
                for y in (row - 1)..=(row + 1) {
                    if special_chars.contains(&(x, y)) {
                        return true;
                    }
                }
            }

            return false;
        })
        .map(|part| {
            (&part.line[part.start_col..=part.end_col])
                .parse::<i32>()
                .unwrap()
        })
        .sum();

    dbg!(valid_engine_parts_sum);
}
