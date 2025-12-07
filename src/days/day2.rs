use std::ops::Range;

pub fn part1<'a>(range_strs: impl Iterator<Item = &'a str>) {
    let mut ranges: Vec<Range<u64>> = range_strs
        .filter_map(|s| {
            let (start, end) = s.split_once('-')?;
            Some(start.parse().ok()?..end.parse::<u64>().ok()? + 1)
        })
        .collect();

    ranges.sort_by_key(|r| r.start);

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

    let total: u64 = merged
        .iter()
        .map(|range| range.clone().filter(is_exactly_twice).sum::<u64>())
        .sum();

    println!("sum of invalid IDs (exactly twice): {total}");
}

fn is_exactly_twice(n: &u64) -> bool {
    let mut digits = [0u8; 20];
    let mut len = 0;
    let mut m = *n;

    if m == 0 {
        return false;
    }

    while m > 0 {
        digits[len] = (m % 10) as u8;
        m /= 10;
        len += 1;
    }

    // Must be even length (pattern repeated exactly twice)
    if len % 2 != 0 {
        return false;
    }

    let half = len / 2;
    digits[..half] == digits[half..len]
}

pub fn part2<'a>(range_strs: impl Iterator<Item = &'a str>) {
    let mut ranges: Vec<Range<u64>> = range_strs
        .filter_map(|s| {
            let (start, end) = s.split_once('-')?;
            Some(start.parse().ok()?..end.parse::<u64>().ok()? + 1) // +1 because Range is exclusive
        })
        .collect();

    // Sort by start position
    ranges.sort_by_key(|r| r.start);

    // Merge overlapping ranges into disjoint ranges
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

    let total: u64 = merged
        .iter_mut()
        .map(|range| range.filter(is_repeating_digits).sum::<u64>())
        .sum();

    println!("sum of invalid IDs: {total}");
}

fn is_repeating_digits(n: &u64) -> bool {
    let mut digits = [0u8; 20];
    let mut len = 0;
    let mut m = n.to_owned();

    if m == 0 {
        return false;
    }

    while m > 0 {
        digits[len] = (m % 10) as u8;
        m /= 10;
        len += 1;
    }

    for pattern_len in 1..=len / 2 {
        // Skip pattern lengths that don't divide len evenly
        if len % pattern_len != 0 {
            continue;
        }

        let pattern = &digits[..pattern_len];
        if digits[pattern_len..len]
            .chunks(pattern_len)
            .all(|chunk| chunk == pattern)
        {
            return true;
        }
    }
    false
}
