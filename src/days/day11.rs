use std::collections::HashMap;

struct Digraph<'a> {
    connections: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Digraph<'a> {
    fn new() -> Self {
        Digraph {
            connections: HashMap::new(),
        }
    }

    fn get_connections(&self, node: &'a str) -> Option<&Vec<&str>> {
        self.connections.get(node)
    }

    fn add_node(&mut self, node: &'a str) {
        self.connections.insert(node, Vec::with_capacity(16));
    }

    fn add_connection(&mut self, node1: &'a str, node2: &'a str) {
        if !self.connections.contains_key(node1) {
            self.add_node(node1);
        }

        self.connections
            .entry(node1)
            .and_modify(|val| val.push(node2));
    }

    fn dp<'b: 'a>(&'b self, paths: &mut HashMap<&'a str, u64>, cur: &'a str, end: &'a str) -> u64 {
        if let Some(p) = paths.get(cur) {
            return *p;
        }

        if cur == end {
            return 1;
        }

        let total = match self.get_connections(cur) {
            Some(neighbors) => neighbors
                .iter()
                .map(|&neighbor| self.dp(paths, neighbor, end))
                .sum(),
            None => 0,
        };
        paths.insert(cur, total);

        total
    }
}

pub fn part1(lines: impl Iterator<Item = String>) {
    let consumed: Vec<String> = lines.collect();
    let mut digraph = Digraph::new();

    for line in consumed.iter() {
        let colon_idx = line.find(':').unwrap();
        let node = &line[..colon_idx];
        digraph.add_node(node);
        for connection in line[colon_idx + 2..].split(' ') {
            digraph.add_connection(node, connection);
        }
        println!(
            "connections for {node}: {:?}",
            digraph.get_connections(node)
        );
    }

    let mut paths = HashMap::new();
    println!(
        "connections to out: {}",
        digraph.dp(&mut paths, "you", "out")
    );
}

pub fn part2(lines: impl Iterator<Item = String>) {
    let consumed: Vec<String> = lines.collect();
    let mut digraph = Digraph::new();

    for line in consumed.iter() {
        let colon_idx = line.find(':').unwrap();
        let node = &line[..colon_idx];
        digraph.add_node(node);
        for connection in line[colon_idx + 2..].split(' ') {
            digraph.add_connection(node, connection);
        }
    }

    // the solution is
    // paths from svr -> fft * fft -> dac * dac -> out
    // +
    // paths from svr -> dac * dac -> fft * fft -> out

    let mut paths = HashMap::new();
    let svr_to_fft_count = digraph.dp(&mut paths, "svr", "fft");
    paths.clear();
    let fft_to_dac_count = digraph.dp(&mut paths, "fft", "dac");
    paths.clear();
    let svr_to_dac_count = digraph.dp(&mut paths, "svr", "dac");
    paths.clear();
    let dac_to_fft_count = digraph.dp(&mut paths, "dac", "fft");
    let fft_to_out_count = digraph.dp(&mut paths, "fft", "out");
    paths.clear();
    let dac_to_out_count = digraph.dp(&mut paths, "dac", "out");

    println!(
        "total paths: {}",
        svr_to_fft_count * fft_to_dac_count * dac_to_out_count
            + svr_to_dac_count * dac_to_fft_count * fft_to_out_count
    );
}
