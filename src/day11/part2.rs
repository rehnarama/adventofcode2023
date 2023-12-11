use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");

    let mut galaxies: HashSet<(usize, usize)> = HashSet::new();
    let mut expanded_galaxies: HashSet<(usize, usize)> = HashSet::new();
    let mut rows_with_galaxies: HashSet<usize> = HashSet::new();
    let mut columns_with_galaxies: HashSet<usize> = HashSet::new();
    let mut width = 0;
    let mut height = 0;

    for (row, line) in input.lines().enumerate() {
        height = height.max(row + 1);
        for (col, c) in line.chars().enumerate() {
            width = width.max(col + 1);

            if c == '#' {
                galaxies.insert((col, row));
                rows_with_galaxies.insert(row);
                columns_with_galaxies.insert(col);
            }
        }
    }

    let mut column_expansion = 0;
    let mut row_expansion = 0;

    for x in 0..width {
        let mut local_row_expansion = 0;
        if !columns_with_galaxies.contains(&x) {
            column_expansion += 1000000 - 1;
        }
        for y in 0..height {
            if !rows_with_galaxies.contains(&y) {
                local_row_expansion += 1000000 - 1;
                row_expansion = row_expansion.max(local_row_expansion);
            }
            if galaxies.contains(&(x, y)) {
                expanded_galaxies.insert((x + column_expansion, y + local_row_expansion));
            }
        }
    }

    // Draw map
    // for y in 0..height {
    //     let mut chars: Vec<char> = vec![];
    //     for x in 0..width {
    //         if galaxies.contains(&(x, y)) {
    //             chars.push('#');
    //         } else {
    //             chars.push('.');
    //         }
    //     }
    //     println!("{}", chars.iter().join(""));
    // }

    // for y in 0..(height + row_expansion) {
    //     let mut chars: Vec<char> = vec![];
    //     for x in 0..(width + column_expansion) {
    //         if expanded_galaxies.contains(&(x, y)) {
    //             chars.push('#');
    //         } else {
    //             chars.push('.');
    //         }
    //     }
    //     println!("{}", chars.iter().join(""));
    // }

    let mut pairs: HashSet<((usize, usize), (usize, usize))> = HashSet::new();
    let mut n = 0;
    for &a in expanded_galaxies.iter() {
        n += 1;
        for &b in expanded_galaxies.iter().skip(n) {
            pairs.insert((a, b));
        }
    }

    let sum: usize = pairs
        .iter()
        .map(|(a, b)| {
            let shortest_path = a.0.abs_diff(b.0) + a.1.abs_diff(b.1);
            shortest_path
        })
        .sum();

    dbg!(sum);
}
