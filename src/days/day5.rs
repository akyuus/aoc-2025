use std::ops::Range;

pub fn part1(mut lines: impl Iterator<Item = String>) {
    let mut ranges: Vec<Range<u64>> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .filter_map(|line| {
            let (start, end) = line.split_once('-')?;
            let s = start.parse::<u64>().ok()?;
            let e = end.parse::<u64>().ok()?;
            Some(s..e + 1)
        })
        .collect();

    ranges.sort_by_key(|r| r.start);

    let mut merged: Vec<Range<u64>> = Vec::new();
    for range in ranges {
        if let Some(last) = merged.last_mut() {
            if range.start <= last.end {
                last.end = last.end.max(range.end);
            } else {
                merged.push(range);
            }
        } else {
            merged.push(range);
        }
    }

    // Check which IDs from second section fall within ranges
    let mut count = 0;
    for line in lines {
        if let Ok(value) = line.parse::<u64>() {
            // Binary search to find if value is in any range
            if merged.binary_search_by(|range| {
                if value < range.start {
                    std::cmp::Ordering::Greater
                } else if value >= range.end {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Equal
                }
            }).is_ok() {
                count += 1;
            }
        }
    }

    println!("IDs within ranges: {count}");
}

pub fn part2(mut lines: impl Iterator<Item = String>) {
    let mut ranges: Vec<Range<u64>> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .filter_map(|line| {
            let (start, end) = line.split_once('-')?;
            let s = start.parse::<u64>().ok()?;
            let e = end.parse::<u64>().ok()?;
            Some(s..e + 1)
        })
        .collect();

    // Sort by start position
    ranges.sort_by_key(|r| r.start);

    // Merge overlapping ranges in one pass
    let mut merged: Vec<Range<u64>> = Vec::new();
    for range in ranges {
        if merged.is_empty() {
            merged.push(range);
            continue;
        }

        let last = merged.last_mut().unwrap();
        if range.start <= last.end {
            last.end = last.end.max(range.end);
        } else {
            merged.push(range);
        }
    }

    let total_fresh_ingredients: u64 = merged.iter().map(|range| range.end - range.start).sum();

    println!("total fresh ingredient IDs: {total_fresh_ingredients}");
}
