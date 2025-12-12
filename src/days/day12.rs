#[derive(Debug, Clone)]
struct Shape {
    area: usize,
}

impl Shape {
    fn parse(lines: &[&str]) -> Self {
        let mut area = 0;
        for line in lines {
            for ch in line.chars() {
                if ch == '#' {
                    area += 1;
                }
            }
        }
        Shape { area }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    width: usize,
    height: usize,
    required_presents: [usize; 6], // Index = shape ID, Value = count needed
}

impl Grid {
    fn new(width: usize, height: usize, required_presents: [usize; 6]) -> Self {
        Grid {
            width,
            height,
            required_presents,
        }
    }
}

fn parse_input(lines: impl Iterator<Item = String>) -> (Vec<Shape>, Vec<Grid>) {
    let lines: Vec<String> = lines.collect();
    let mut i = 0;
    let mut shapes = Vec::new();
    let mut grids = Vec::new();

    while i < lines.len() {
        let line = lines[i].trim();

        if line.contains('x') && line.contains(':') {
            break;
        }

        if line.is_empty() {
            i += 1;
            continue;
        }

        if line.ends_with(':') {
            if i + 3 < lines.len() {
                let shape_lines = [
                    lines[i + 1].as_str(),
                    lines[i + 2].as_str(),
                    lines[i + 3].as_str(),
                ];
                shapes.push(Shape::parse(&shape_lines));
                i += 4; // Skip the "N:" line and 3 shape lines
            } else {
                break;
            }
        } else {
            i += 1;
        }
    }

    while i < lines.len() {
        let line = lines[i].trim();

        if line.is_empty() {
            i += 1;
            continue;
        }
        let (dimensions, counts) = line.split_once(':').unwrap();
        let (width_str, height_str) = dimensions.split_once('x').unwrap();
        let width = width_str.trim().parse().unwrap();
        let height = height_str.trim().parse().unwrap();
        let mut required_presents: [usize; 6] = [0; 6];
        let required_presents_vec: Vec<usize> = counts
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        required_presents.copy_from_slice(&required_presents_vec);

        grids.push(Grid::new(width, height, required_presents));

        i += 1;
    }

    (shapes, grids)
}

pub fn part1(lines: impl Iterator<Item = String>) -> usize {
    let (shapes, grids) = parse_input(lines);

    let mut solvable_count = 0;

    for grid in grids {
        let needed_area: usize = grid
            .required_presents
            .iter()
            .enumerate()
            .map(|(i, &count)| count * shapes[i].area)
            .sum();

        let grid_area = grid.width * grid.height;

        if needed_area < grid_area {
            solvable_count += 1;
        }
    }

    println!("total solvable: {solvable_count}");
    solvable_count
}

pub fn part2(lines: impl Iterator<Item = String>) -> usize {
    0
}
