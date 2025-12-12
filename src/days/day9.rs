pub fn part1(lines: impl Iterator<Item = String>) -> u64 {
    let mut red_tiles = Vec::new();
    for line in lines {
        let mut coords_iter = line.split(',').map(|s| s.parse::<u64>().unwrap());
        let x = coords_iter.next().unwrap();
        let y = coords_iter.next().unwrap();
        red_tiles.push((x, y));
    }
    let mut areas = Vec::with_capacity(red_tiles.len() * (red_tiles.len() - 1) / 2);
    for i in 0..red_tiles.len() {
        for j in (i + 1)..red_tiles.len() {
            areas.push(manhattan_distance(red_tiles[i], red_tiles[j]));
        }
    }

    let max = areas.iter().max().unwrap();
    println!("max area: {max}");
    *max
}

fn manhattan_distance(a: (u64, u64), b: (u64, u64)) -> u64 {
    (a.0 as i64 - b.0 as i64 + 1).unsigned_abs() * (a.1 as i64 - b.1 as i64 + 1).unsigned_abs()
}

pub fn part2(lines: impl Iterator<Item = String>) -> u64 {
    let red_tiles: Vec<(u64, u64)> = lines
        .map(|line| {
            let mut coords_iter = line.split(',').map(|s| s.parse::<u64>().unwrap());
            let x = coords_iter.next().unwrap();
            let y = coords_iter.next().unwrap();
            (x, y)
        })
        .collect();

    // Step 1: Extract and sort unique coordinates
    let mut xs: Vec<u64> = red_tiles.iter().map(|p| p.0).collect();
    xs.sort();
    xs.dedup();

    let mut ys: Vec<u64> = red_tiles.iter().map(|p| p.1).collect();
    ys.sort();
    ys.dedup();

    let get_x = |val: u64| xs.binary_search(&val).unwrap();
    let get_y = |val: u64| ys.binary_search(&val).unwrap();

    // Step 2: Create wall grid for vertical edges
    let mut wall = vec![vec![false; xs.len()]; ys.len() - 1];

    let n = red_tiles.len();
    for i in 0..n {
        let p1 = red_tiles[i];
        let p2 = red_tiles[(i + 1) % n]; // Wrap around to close the loop

        if p1.0 == p2.0 {
            let xi = get_x(p1.0);
            let y_start = get_y(p1.1.min(p2.1));
            let y_end = get_y(p1.1.max(p2.1));
            for j in y_start..y_end {
                wall[j][xi] = true;
            }
        }
    }

    // Step 3: Scanline algorithm to mark inside regions
    let mut g = vec![vec![0u8; xs.len()]; ys.len() - 1];

    for j in 0..ys.len() - 1 {
        let mut inside = false;
        for i in 0..xs.len() {
            if wall[j][i] {
                inside = !inside; //
            }
            g[j][i] = inside as u8;
        }
    }

    println!("Inside/Outside grid:");
    for row in &g {
        println!("{:?}", row);
    }

    let mut max_area = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            let (x1, y1) = red_tiles[i];
            let (x2, y2) = red_tiles[j];

            let xi1 = get_x(x1).min(get_x(x2));
            let xi2 = get_x(x1).max(get_x(x2));
            let yi1 = get_y(y1).min(get_y(y2));
            let yi2 = get_y(y1).max(get_y(y2));

            if xi1 == xi2 || yi1 == yi2 {
                continue;
            }

            let mut all_inside = true;
            for y in yi1..yi2 {
                for x in xi1..xi2 {
                    if g[y][x] != 1 {
                        all_inside = false;
                        break;
                    }
                }
                if !all_inside {
                    break;
                }
            }

            if all_inside {
                let actual_area = ((x2 as i64 - x1 as i64).abs() + 1) as u64
                    * ((y2 as i64 - y1 as i64).abs() + 1) as u64;
                max_area = max_area.max(actual_area);
            }
        }
    }

    println!("max area: {max_area}");
    max_area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_example() {
        let input = r##"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"##;

        let lines = input.lines().map(String::from);
        assert!(part1(lines) == 50, "incorrect result");
    }
}
