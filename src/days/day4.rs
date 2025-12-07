pub fn part1(grid: Vec<Vec<u8>>) {
    let mut count: u32 = 0;
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, val) in row.iter().enumerate() {
            if *val != b'@' {
                continue;
            }
            if is_accessible(&grid, row_idx, col_idx) {
                count += 1;
            }
        }
    }
    println!("accessible rolls: {count}");
}

pub fn part2(mut grid: Vec<Vec<u8>>) {
    let mut total_count: u32 = 0;
    let mut removables = Vec::new();
    let mut iter = 0;

    loop {
        iter += 1;
        removables.clear();

        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, val) in row.iter().enumerate() {
                if *val != b'@' {
                    continue;
                }
                if is_accessible(&grid, row_idx, col_idx) {
                    removables.push((row_idx, col_idx));
                }
            }
        }

        let count = removables.len() as u32;
        println!("removing rolls on iteration {iter}: {count}");
        total_count += count;

        if count == 0 {
            break;
        }

        for (r_idx, c_idx) in &removables {
            grid[*r_idx][*c_idx] = b'.';
        }
    }
    println!("total count: {total_count}");
}

fn is_accessible(grid: &[Vec<u8>], row: usize, col: usize) -> bool {
    let mut rolls: u8 = 0;

    // 8 directions: (row_offset, col_offset)
    const DIRECTIONS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1), // top row
        (0, -1),
        (0, 1), // middle row (skip center)
        (1, -1),
        (1, 0),
        (1, 1), // bottom row
    ];

    for (dr, dc) in DIRECTIONS {
        let new_row = row as isize + dr;
        let new_col = col as isize + dc;

        // Check bounds
        if new_row < 0
            || new_col < 0
            || new_row >= grid.len() as isize
            || new_col >= grid[0].len() as isize
        {
            continue;
        }

        let cell = grid[new_row as usize][new_col as usize];
        if cell == b'@' {
            rolls += 1;
        }
        if rolls >= 4 {
            return false;
        }
    }

    true
}
