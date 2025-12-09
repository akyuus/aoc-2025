use std::collections::{HashMap, HashSet};

pub fn part1(lines: impl Iterator<Item = String>) {
    let grid: Vec<Vec<u8>> = lines.map(|s| s.into_bytes()).collect();

    let start_col = grid[0].iter().position(|&b| b == b'S').unwrap();

    let mut split_count = 0;

    let mut beams = HashSet::with_capacity(grid.len());
    beams.insert(start_col);

    for row in grid.iter() {
        let mut new_cols = HashSet::with_capacity(grid.len());
        for col in beams.iter() {
            if row[*col] != b'^' {
                new_cols.insert(*col);
                continue;
            }

            // Hit a splitter
            split_count += 1;
            // Create two new downward beams from left and right of splitter
            if *col > 0 {
                new_cols.insert(*col - 1);
            }
            if *col < row.len() - 1 {
                new_cols.insert(*col + 1);
            }
        }
        beams = new_cols;
    }

    println!("split count: {split_count}");
}

pub fn part2(lines: impl Iterator<Item = String>) {
    let grid: Vec<Vec<u8>> = lines.map(|s| s.into_bytes()).collect();

    let start_col = grid[0].iter().position(|&b| b == b'S').unwrap();

    let mut beams = vec![0; grid[0].len()];
    beams[start_col] = 1;

    for row in grid.iter() {
        for col in 0..beams.len() {
            let timeline = beams[col];
            if timeline == 0 || row[col] != b'^' {
                continue;
            }

            // Hit a splitter
            // Create two new downward beams from left and right of splitter
            if col > 0 {
                beams[col - 1] += timeline;
            }
            if col < row.len() - 1 {
                beams[col + 1] += timeline;
            }
            beams[col] = 0;
        }
    }

    println!("total timelines: {}", beams.iter().sum::<u64>());
}
