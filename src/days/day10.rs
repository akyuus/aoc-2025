use std::collections::HashSet;

pub fn part1(lines: impl Iterator<Item = String>) {
    let input: Vec<String> = lines.collect();
    let mut masks = Vec::with_capacity(input.len());
    let mut target_masks: Vec<u16> = Vec::with_capacity(input.len());
    for line in input.iter() {
        // we don't care about joltage requirement for now
        let mut line_iter = line.split(' ');
        let target = line_iter.next().unwrap();
        let bit_length = target.len() - 2;
        let mut curr_target_mask = 0;
        for (i, c) in target[1..target.len() - 1]
            .as_bytes()
            .iter()
            .rev()
            .enumerate()
        {
            if *c == b'#' {
                curr_target_mask ^= 1 << i;
            }
        }
        println!("target mask: {:#016b}", curr_target_mask);
        target_masks.push(curr_target_mask);

        let mut curr_masks = Vec::with_capacity(16);
        for mask in line_iter {
            if mask.as_bytes()[0] == b'{' {
                break;
            }

            let bytes: Vec<u8> = mask[1..mask.len() - 1]
                .split(',')
                .map(|s| s.as_bytes()[0] - b'0')
                .collect();
            println!("bytes: {}", String::from_utf8(bytes.clone()).unwrap());
            println!("bit length: {bit_length}");
            curr_masks.push(slice_to_u16_mask(&bytes, bit_length));
        }
        for mask in curr_masks.iter() {
            println!("saw mask: {:#016b}", mask);
        }
        masks.push(curr_masks);
    }

    // we have all of our masks and targets. to actually solve the problem, we have to track what our current value is at and how many times we've applied a mask
    let mut counts = Vec::with_capacity(input.len());
    for i in 0..target_masks.len() {
        let target_mask = target_masks[i];
        let curr_masks = &masks[i];
        let mut curr_values = vec![HashSet::new()];
        curr_values[0].insert(0);
        let mut count = 0;
        'outer: loop {
            let mut new_iteration = HashSet::new();
            for past_val in curr_values.last().unwrap().iter() {
                for mask in curr_masks {
                    let res = mask ^ past_val;
                    if res == target_mask {
                        counts.push(count + 1);
                        break 'outer;
                    }
                    new_iteration.insert(res);
                }
            }
            curr_values.push(new_iteration);
            count += 1;
        }
    }
    println!("counts: {:?}, sum: {}", counts, counts.iter().sum::<u32>());
}

fn slice_to_u16_mask(v: &[u8], bit_length: usize) -> u16 {
    let mut mask = 0;
    for val in v {
        // val means "flip the valth" digit from the right
        // so if bit_length = 6, and val is 3
        // mask = 0000000000000000
        //                  ^start here
        // flip the 3rd digit, which would equate to
        // 1 << (6 - 3 - 1)
        mask ^= 1 << (bit_length - *val as usize - 1);
    }
    mask
}

pub fn part2(lines: impl Iterator<Item = String>) {
    let input: Vec<String> = lines.collect();
    let mut masks = Vec::with_capacity(input.len());
    let mut joltages = Vec::<Vec<u16>>::new();

    for line in input.iter() {
        let line_iter = line.split(' ');

        let mut curr_masks = Vec::with_capacity(16);
        for piece in line_iter {
            if piece.as_bytes()[0] == b'[' {
                continue;
            }
            if piece.as_bytes()[0] == b'{' {
                let mut joltage = Vec::new();
                for c in piece[1..piece.len() - 1].split(',') {
                    joltage.push(c.parse::<u16>().unwrap());
                }
                // println!("joltages: {:?}", joltage);
                joltages.push(joltage);
                continue;
            }

            let bytes: Vec<u8> = piece[1..piece.len() - 1]
                .split(',')
                .map(|s| s.as_bytes()[0] - b'0')
                .collect();
            curr_masks.push(bytes);
        }
        masks.push(curr_masks);
    }

    let mut counts = Vec::with_capacity(joltages.len());
    for (i, joltage) in joltages.iter().enumerate() {
        counts.push(find_depth(joltage, &masks[i]));
    }
    println!("counts: {:?}, sum: {}", counts, counts.iter().sum::<i64>());
}

fn find_depth(joltages: &[u16], buttons: &[Vec<u8>]) -> i64 {
    let n_counters = joltages.len();
    let n_buttons = buttons.len();

    let mut matrix: Vec<Vec<i64>> = vec![vec![0; n_buttons + 1]; n_counters];

    for (button_idx, button) in buttons.iter().enumerate() {
        for &counter_idx in button {
            matrix[counter_idx as usize][button_idx] = 1;
        }
    }

    // Fill in the RHS (target values)
    for (i, &target) in joltages.iter().enumerate() {
        matrix[i][n_buttons] = target as i64;
    }

    let max_joltage = *joltages.iter().max().unwrap_or(&0) as i64;

    // Solve the system using Gaussian elimination
    let solution = solve_linear_system(&matrix, n_buttons, max_joltage);
    let total_presses: i64 = solution.iter().sum();
    total_presses
}

fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a.unsigned_abs();
    let mut b = b.unsigned_abs();

    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a.max(1) as i64
}

fn gcd_vec(vec: &[i64]) -> i64 {
    vec.iter().fold(0, |acc, &x| gcd(acc, x))
}

fn solve_linear_system(matrix: &[Vec<i64>], n_vars: usize, max_joltage: i64) -> Vec<i64> {
    let n_equations = matrix.len();
    let mut matrix = matrix.to_vec();

    // Gaussian elimination to RREF
    let mut pivot_row = 0;
    let mut dependent_cols = Vec::new();
    let mut independent_cols = Vec::new();

    for col in 0..n_vars {
        if pivot_row >= n_equations {
            independent_cols.push(col);
            continue;
        }

        // Find non-zero pivot
        if let Some(best_row) = (pivot_row..n_equations).find(|&r| matrix[r][col] != 0) {
            // Swap rows
            matrix.swap(pivot_row, best_row);
            dependent_cols.push(col);

            // Scale pivot row by GCD
            let row_gcd = gcd_vec(&matrix[pivot_row]);
            if row_gcd > 1 {
                for c in 0..=n_vars {
                    matrix[pivot_row][c] /= row_gcd;
                }
            }

            let pivot_val = matrix[pivot_row][col];

            // Eliminate column in all other rows (RREF)
            for row in 0..n_equations {
                if row != pivot_row && matrix[row][col] != 0 {
                    let factor = matrix[row][col];

                    for c in 0..=n_vars {
                        matrix[row][c] = matrix[row][c] * pivot_val - matrix[pivot_row][c] * factor;
                    }

                    // Reduce by GCD
                    let row_gcd = gcd_vec(&matrix[row]);
                    if row_gcd > 1 {
                        for c in 0..=n_vars {
                            matrix[row][c] /= row_gcd;
                        }
                    }
                }
            }

            pivot_row += 1;
        } else {
            independent_cols.push(col);
        }
    }

    // DFS to find minimum solution
    let max_val = max_joltage + 1;
    let mut min_total = i64::MAX;
    let mut best_solution = vec![0i64; n_vars];
    let mut indep_values = vec![0i64; independent_cols.len()];

    dfs_search(
        &matrix,
        &dependent_cols,
        &independent_cols,
        0,
        &mut indep_values,
        &mut min_total,
        &mut best_solution,
        max_val as usize,
    );

    if min_total == i64::MAX {
        vec![0; n_vars]
    } else {
        best_solution
    }
}

fn dfs_search(
    matrix: &[Vec<i64>],
    dependent_cols: &[usize],
    independent_cols: &[usize],
    idx: usize,
    indep_values: &mut [i64],
    min_total: &mut i64,
    best_solution: &mut [i64],
    max_val: usize,
) {
    let n_vars = matrix[0].len() - 1;

    if idx == independent_cols.len() {
        if let Some(total) = validate_and_compute(
            matrix,
            dependent_cols,
            independent_cols,
            indep_values,
            n_vars,
        ) {
            if total < *min_total {
                *min_total = total;
                // Reconstruct full solution
                (0..n_vars).for_each(|i| {
                    best_solution[i] = 0;
                });
                for (i, &col) in independent_cols.iter().enumerate() {
                    best_solution[col] = indep_values[i];
                }
                for (row, &col) in dependent_cols.iter().enumerate() {
                    let pivot = matrix[row][col];
                    let mut rhs = matrix[row][n_vars];
                    for (i, &indep_col) in independent_cols.iter().enumerate() {
                        rhs -= matrix[row][indep_col] * indep_values[i];
                    }
                    best_solution[col] = rhs / pivot;
                }
            }
        }
        return;
    }

    // Pruning: if current partial sum already exceeds minimum, stop
    let partial_sum: i64 = indep_values[..idx].iter().sum();

    for val in 0..max_val as i64 {
        if partial_sum + val >= *min_total {
            break;
        }
        indep_values[idx] = val;
        dfs_search(
            matrix,
            dependent_cols,
            independent_cols,
            idx + 1,
            indep_values,
            min_total,
            best_solution,
            max_val,
        );
    }
}

fn validate_and_compute(
    matrix: &[Vec<i64>],
    dependent_cols: &[usize],
    independent_cols: &[usize],
    indep_values: &[i64],
    n_vars: usize,
) -> Option<i64> {
    let mut total: i64 = indep_values.iter().sum();

    // For each dependent variable (pivot), compute its value
    for (row, &col) in dependent_cols.iter().enumerate() {
        let pivot = matrix[row][col];
        let mut rhs = matrix[row][n_vars];

        // Subtract contributions from independent (free) variables
        for (i, &indep_col) in independent_cols.iter().enumerate() {
            rhs -= matrix[row][indep_col] * indep_values[i];
        }

        // Check if we get a valid non-negative integer
        if rhs % pivot != 0 {
            return None;
        }

        let val = rhs / pivot;
        if val < 0 {
            return None;
        }

        total += val;
    }

    Some(total)
}
