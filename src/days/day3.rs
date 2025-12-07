pub fn part1(banks: impl Iterator<Item = String>) {
    let mut total_joltage: u64 = 0;
    for bank in banks {
        if bank.is_empty() {
            continue;
        }
        let bytes = bank.as_bytes();

        let (idx1, digit1) = get_max_digit(bytes, -1, bytes.len() - 1);

        let (_, digit2) = get_max_digit(bytes, idx1 as i16, bytes.len());

        let joltage = u64::from(digit1) * 10 + u64::from(digit2);
        total_joltage += joltage;
    }
    println!("total joltage (2 digits): {}", total_joltage);
}

pub fn part2(banks: impl Iterator<Item = String>) {
    let mut total_joltage: u64 = 0;
    for bank in banks {
        if bank.is_empty() {
            continue;
        }
        let bytes = bank.as_bytes();
        let mut max_joltage_digits = [0; 12];
        let mut left: i16 = -1;
        let mut right = bank.len() - 11;
        (0..12).for_each(|i| {
            let (idx, max) = get_max_digit(bytes, left, right);
            max_joltage_digits[i] = max;
            left = idx as i16;
            right += 1;
        });
        let max_joltage = max_joltage_digits.iter().fold(0_u64, |mut acc, digit| {
            acc *= 10;
            acc += u64::from(*digit);
            acc
        });
        total_joltage += max_joltage;
    }
    println!("total joltage: {}", total_joltage);
}

fn get_max_digit(digits: &[u8], left: i16, right: usize) -> (usize, u8) {
    let mut max = 0;
    let mut max_idx = 0;
    let corrected_left = (left + 1) as usize;
    for i in corrected_left..right {
        let d = digits[i];
        if d == b'9' {
            max = 9;
            max_idx = i;
            break;
        }

        let val = d - b'0';
        if val > max {
            max = val;
            max_idx = i;
        }
    }
    (max_idx, max)
}
