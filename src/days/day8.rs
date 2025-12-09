use std::collections::HashMap;

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return;
        }

        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }
    }

    fn get_component_sizes(&mut self, n: usize) -> Vec<usize> {
        let mut sizes: HashMap<usize, usize> = HashMap::new();
        for i in 0..n {
            let root = self.find(i);
            *sizes.entry(root).or_insert(0) += 1;
        }
        sizes.values().copied().collect()
    }

    fn all_connected(&mut self) -> bool {
        let mut root_iter = (0..self.parent.len()).map(|idx| self.find(idx));
        let first = root_iter.next().unwrap();
        root_iter.all(|x| x == first)
    }
}

pub fn part1(lines: impl Iterator<Item = String>) -> usize {
    let points: Vec<(u64, u64, u64)> = lines
        .map(|x| {
            let mut nums = x.split(',').map(|num| num.parse::<u64>().unwrap());
            let a = nums.next().unwrap();
            let b = nums.next().unwrap();
            let c = nums.next().unwrap();
            (a, b, c)
        })
        .collect();
    let mut distances: Vec<(usize, usize, u64)> =
        Vec::with_capacity(points.len() * (points.len() - 1) / 2);
    for (i, p1) in points.iter().enumerate() {
        for j in (i + 1)..points.len() {
            let p2 = points[j];
            let magnitude = ((p1.0 as i64 - p2.0 as i64).pow(2)
                + (p1.1 as i64 - p2.1 as i64).pow(2)
                + (p1.2 as i64 - p2.2 as i64).pow(2)) as u64;
            distances.push((i, j, magnitude));
        }
    }
    distances.sort_by(|a, b| a.2.cmp(&b.2));

    let mut uf = UnionFind::new(points.len());

    for (i, j, _) in &distances[0..1000] {
        uf.union(*i, *j);
    }

    let mut sizes = uf.get_component_sizes(points.len());
    sizes.sort_by(|a, b| b.cmp(a));

    let result = sizes[0] * sizes[1] * sizes[2];
    println!("result: {result}");
    result
}

pub fn part2(lines: impl Iterator<Item = String>) -> u64 {
    {
        let points: Vec<(u64, u64, u64)> = lines
            .map(|x| {
                let mut nums = x.split(',').map(|num| num.parse::<u64>().unwrap());
                let a = nums.next().unwrap();
                let b = nums.next().unwrap();
                let c = nums.next().unwrap();
                (a, b, c)
            })
            .collect();
        let mut distances: Vec<(usize, usize, u64)> =
            Vec::with_capacity(points.len() * (points.len() - 1) / 2);
        for (i, p1) in points.iter().enumerate() {
            for j in (i + 1)..points.len() {
                let p2 = points[j];
                let magnitude = ((p1.0 as i64 - p2.0 as i64).pow(2)
                    + (p1.1 as i64 - p2.1 as i64).pow(2)
                    + (p1.2 as i64 - p2.2 as i64).pow(2)) as u64;
                distances.push((i, j, magnitude));
            }
        }
        distances.sort_by(|a, b| a.2.cmp(&b.2));

        let mut uf = UnionFind::new(points.len());

        let mut result: u64 = 0;
        for (i, j, _) in &distances[..] {
            uf.union(*i, *j);
            if uf.all_connected() {
                result = points[*i].0 * points[*j].0;
                break;
            }
        }

        println!("result: {result}");
        result
    }
}
