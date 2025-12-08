use std::collections::{HashMap, HashSet};

pub fn part1(lines: impl Iterator<Item = String>) {
    let grid: Vec<Vec<u8>> = lines.map(|s| s.into_bytes()).collect();

    let start_col = grid[0].iter().position(|&b| b == b'S').unwrap();

    let mut split_count = 0;
    let mut split_positions = HashSet::new();

    let mut beams: Vec<(usize, usize)> = vec![(0, start_col)];

    while let Some((row, col)) = beams.pop() {
        for r in row..grid.len() {
            if grid[r][col] != b'^' {
                continue;
            }

            // Hit a splitter
            if split_positions.insert((r, col)) {
                split_count += 1;
                // Create two new downward beams from left and right of splitter
                if col > 0 {
                    beams.push((r, col - 1));
                }
                if col < grid[r].len() - 1 {
                    beams.push((r, col + 1));
                }
            }
            break;
        }
    }

    println!("split count: {split_count}");
}

pub fn part2(lines: impl Iterator<Item = String>) {
    let grid: Vec<Vec<u8>> = lines.map(|s| s.into_bytes()).collect();

    let start_col = grid[0].iter().position(|&b| b == b'S').unwrap();

    let total = part2_dp(&grid, 0, start_col, &mut HashMap::new());

    println!("split count: {total}");
}

fn part2_dp(
    grid: &Vec<Vec<u8>>,
    row: usize,
    col: usize,
    cache: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if row >= grid.len() {
        return 1;
    }

    if let Some(v) = cache.get(&(row, col)) {
        return *v;
    }

    if grid[row][col] == b'^' {
        let v = part2_dp(grid, row, col - 1, cache) + part2_dp(grid, row, col + 1, cache);
        cache.insert((row, col), v);
        return v;
    }

    let v = part2_dp(grid, row + 1, col, cache);
    cache.insert((row, col), v);
    v
}
