pub fn part1(lines: Vec<String>) {
    // we actually want to reverse the iterator so we can get the operators first.
    // start by grabbing the last line and grabbing all the operators, we can track them in a vec

    let operators: Vec<&str> = lines
        .iter()
        .next_back()
        .unwrap()
        .split_whitespace()
        .map(|x| x.trim())
        .collect();

    // with the operators, we can can process the rows more easily by just maintaining a vec of defined length and calculating as we go
    let mut results = vec![0; operators.len()];
    for (i, &op) in operators.iter().enumerate() {
        match op {
            "+" => results[i] = 0,
            "*" => results[i] = 1,
            _ => panic!("unreachable"),
        }
    }
    for line in lines.iter().rev().skip(1) {
        let numbers = line.split_whitespace().map(|x| x.trim());
        for (i, number) in numbers.enumerate() {
            let num_i64 = number.parse::<i64>().unwrap();
            match operators[i] {
                "+" => results[i] += num_i64,
                "*" => results[i] *= num_i64,
                _ => panic!("unreachable"),
            }
        }
    }
    println!("grand total: {}", results.iter().sum::<i64>());
}

pub fn part2(lines: Vec<String>) {
    let mut operators = vec![];
    for (i, c) in lines
        .iter()
        .next_back()
        .unwrap()
        .as_bytes()
        .iter()
        .enumerate()
    {
        if c.is_ascii_whitespace() {
            continue;
        }
        operators.push((i, *c));
    }

    let mut results = vec![0; operators.len()];
    for (i, (_, op)) in operators.iter().enumerate() {
        match op {
            b'+' => results[i] = 0,
            b'*' => results[i] = 1,
            _ => panic!("unreachable"),
        }
    }

    for (idx, (i, op)) in operators.iter().enumerate() {
        let end = if idx == operators.len() - 1 {
            lines[0].len()
        } else {
            operators[idx + 1].0 - 1
        };
        for col in *i..end {
            let mut val: i64 = 0;
            (0..lines.len()).for_each(|row| match lines[row].as_bytes()[col] {
                c if c.is_ascii_digit() => {
                    val *= 10;
                    val += (c - b'0') as i64;
                }
                _ => (),
            });
            println!("val: {val}");
            match op {
                b'+' => results[idx] += val,
                b'*' => results[idx] *= val,
                _ => panic!("unreachable"),
            }
        }
    }
    println!("final total: {}", results.iter().sum::<i64>());
}
