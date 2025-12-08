use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

struct Edge {
    dist: f64,
    a: usize,
    b: usize,
}

// ----------------------------
// DSU / Union-Find
// ----------------------------
struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
    components: usize,
}

impl DSU {
    fn new(n: usize) -> Self {
        DSU {
            parent: (0..n).collect(),
            size: vec![1; n],
            components: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut pa = self.find(a);
        let mut pb = self.find(b);

        if pa == pb {
            return false;
        }

        if self.size[pa] < self.size[pb] {
            std::mem::swap(&mut pa, &mut pb);
        }

        self.parent[pb] = pa;
        self.size[pa] += self.size[pb];

        self.components -= 1;
        true
    }
}

fn main() {
    // ----------------------------
    // Read input
    // ----------------------------
    let file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut points = Vec::new();

    for line in reader.lines() {
        let l = line.unwrap();
        if l.trim().is_empty() {
            continue;
        }
        let nums: Vec<f64> = l.split(',').map(|v| v.parse().unwrap()).collect();
        points.push(Point { x: nums[0], y: nums[1], z: nums[2] });
    }

    let n = points.len();
    println!("Loaded {n} points.");

    // ----------------------------
    // Compute all pairwise distances
    // ----------------------------
    let mut edges = Vec::new();

    for i in 0..n {
        for j in (i + 1)..n {
            let dx = points[i].x - points[j].x;
            let dy = points[i].y - points[j].y;
            let dz = points[i].z - points[j].z;
            let d = (dx*dx + dy*dy + dz*dz).sqrt();

            edges.push(Edge { dist: d, a: i, b: j });
        }
    }

    // Sort distances
    edges.sort_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap());

    // ----------------------------
    // PART 2:
    // Continue joining until EVERYTHING is connected
    // ----------------------------
    let mut dsu = DSU::new(n);

    let mut last_a = 0;
    let mut last_b = 0;

    for e in edges {
        if dsu.union(e.a, e.b) {
            // This was a successful component merge
            last_a = e.a;
            last_b = e.b;

            if dsu.components == 1 {
                // All connected!
                break;
            }
        }
    }

    println!("Final merge was between points:");
    println!("A: ({}, {}, {})", points[last_a].x, points[last_a].y, points[last_a].z);
    println!("B: ({}, {}, {})", points[last_b].x, points[last_b].y, points[last_b].z);

    let answer = (points[last_a].x as i64) * (points[last_b].x as i64);
    println!("ANSWER Part 2 = {answer}");
}
