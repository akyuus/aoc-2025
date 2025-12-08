use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
};

mod days;
use days::*;

fn main() -> io::Result<()> {
    let day = env::args()
        .nth(1)
        .expect("Usage: cargo run -- <day> <part>");
    let part: u8 = env::args()
        .nth(2)
        .expect("Usage: cargo run -- <day> <part>")
        .parse()
        .expect("Part must be 1 or 2");

    match day.as_str() {
        "day1" => {
            let file = File::open("input/day1")?;
            let reader = BufReader::new(file);
            match part {
                1 => day1::part1(reader.lines().map_while(Result::ok)),
                2 => day1::part2(reader.lines().map_while(Result::ok)),
                _ => panic!("Part must be 1 or 2"),
            }
        }
        "day2" => {
            let file = File::open("input/day2")?;
            let mut reader = BufReader::new(file);
            let mut line = String::new();
            reader.read_line(&mut line)?;
            match part {
                1 => day2::part1(line.trim().split(',')),
                2 => day2::part2(line.trim().split(',')),
                _ => panic!("Part must be 1 or 2"),
            }
        }
        "day3" => {
            let file = File::open("input/day3")?;
            let reader = BufReader::new(file);
            match part {
                1 => day3::part1(reader.lines().map_while(Result::ok)),
                2 => day3::part2(reader.lines().map_while(Result::ok)),
                _ => panic!("Part must be 1 or 2"),
            }
        }
        "day4" => {
            let file = File::open("input/day4")?;
            let reader = BufReader::new(file);
            let grid: Vec<Vec<u8>> = reader
                .lines()
                .map_while(Result::ok)
                .map(|s| s.into_bytes())
                .collect();
            match part {
                1 => day4::part1(grid),
                2 => day4::part2(grid),
                _ => panic!("Part must be 1 or 2"),
            }
        }
        "day5" => {
            let file = File::open("input/day5")?;
            let reader = BufReader::new(file);
            match part {
                1 => day5::part1(reader.lines().map_while(Result::ok)),
                2 => day5::part2(reader.lines().map_while(Result::ok)),
                _ => panic!("Part must be 1 or 2"),
            }
        }
        "day6" => {
            let file = File::open("input/day6")?;
            let reader = BufReader::new(file);
            let lines: Vec<String> = reader
                .lines()
                .map_while(Result::ok)
                .filter(|x| !x.trim().is_empty())
                .collect();
            match part {
                1 => day6::part1(lines),
                2 => day6::part2(lines),
                _ => panic!("Part must be 1 or 2"),
            }
        }
        "day7" => {
            let file = File::open("input/day7")?;
            let reader = BufReader::new(file);
            let lines = reader.lines().map_while(Result::ok);
            match part {
                1 => day7::part1(lines),
                2 => day7::part2(lines),
                _ => panic!("Part must be 1 or 2"),
            }
        }
        _ => unimplemented!(),
    }

    Ok(())
}
