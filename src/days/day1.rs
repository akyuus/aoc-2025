pub fn part1(lines: impl Iterator<Item = String>) {
    let mut total_zeroes: u32 = 0;
    let mut val: i32 = 50;

    for line in lines {
        let bytes = line.as_bytes();
        let mul = if bytes[0] == b'L' { -1 } else { 1 };

        let curr_val = bytes.iter().skip(1).fold(0_i32, |acc, &x| {
            acc * 10 + i32::from(x - b'0')
        });

        val += (curr_val % 100) * mul;
        val = val.rem_euclid(100);

        if val == 0 {
            total_zeroes += 1;
        }
    }
    println!("total landings on zero: {total_zeroes}");
}

pub fn part2(lines: impl Iterator<Item = String>) {
    let mut total_zeroes: u32 = 0;
    let mut val: i32 = 50;

    for line in lines {
        let bytes = line.as_bytes();
        let mul = if bytes[0] == b'L' { -1 } else { 1 };

        let curr_val = bytes.iter().skip(1).fold(0_i32, |acc, &x| {
            acc * 10 + i32::from(x - b'0')
        });

        let mut bonus_zero_rotations = curr_val / 100;
        let prev_val = val;

        val += (curr_val % 100) * mul;
        if !(1..100).contains(&val) && prev_val != 0 {
            bonus_zero_rotations += 1;
        }

        val = val.rem_euclid(100);
        total_zeroes += bonus_zero_rotations as u32;
    }
    println!("total zeroes: {total_zeroes}");
}
