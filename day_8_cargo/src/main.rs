use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Clone)]
struct Edge {
    dist: f64,
    a: usize,
    b: usize,
}

// ----------------------------
// Union-Find / DSU
// ----------------------------
struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        DSU {
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

    fn union(&mut self, a: usize, b: usize) {
        let mut pa = self.find(a);
        let mut pb = self.find(b);

        if pa == pb { return; }

        if self.size[pa] < self.size[pb] {
            std::mem::swap(&mut pa, &mut pb);
        }

        self.parent[pb] = pa;
        self.size[pa] += self.size[pb];
    }
}

// ----------------------------
// MAIN
// ----------------------------
fn main() {

    let file_path = "src/input.txt";
    let file = File::open(file_path).expect("Cannot open input.txt");
    let reader = BufReader::new(file);

    // 1) Leer puntos
    let mut points: Vec<Point> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            continue;
        }
        let nums: Vec<f64> = line
            .split(',')
            .map(|v| v.parse::<f64>().unwrap())
            .collect();

        points.push(Point {
            x: nums[0],
            y: nums[1],
            z: nums[2],
        });
    }

    let n = points.len();
    println!("Loaded {n} points.");

    // 2) Calcular TODAS las distancias (N^2)
    let mut edges: Vec<Edge> = Vec::new();

    for i in 0..n {
        for j in (i + 1)..n {
            let dx = points[i].x - points[j].x;
            let dy = points[i].y - points[j].y;
            let dz = points[i].z - points[j].z;
            let d = (dx*dx + dy*dy + dz*dz).sqrt();

            edges.push(Edge { dist: d, a: i, b: j });
        }
    }

    // 3) Ordenar por distancia creciente
    edges.sort_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap());

    // 4) Tomar SOLO las 1000 distancias más cortas
    let k = 1000;
    let limit = k.min(edges.len());

    let mut dsu = DSU::new(n);

    for i in 0..limit {
        let e = &edges[i];
        dsu.union(e.a, e.b);
    }

    // 5) Calcular tamaños finales por componente raíz
    let mut comp_size = vec![0usize; n];

    for i in 0..n {
        let root = dsu.find(i);
        comp_size[root] += 1;
    }

    // 6) Filtrar tamaños > 0
    let mut sizes: Vec<usize> = comp_size.into_iter().filter(|&x| x > 0).collect();

    // 7) Ordenar tamaños de mayor a menor
    sizes.sort();
    sizes.reverse();

    println!("Largest components: {:?}", &sizes[0..3]);

    let answer = sizes[0] * sizes[1] * sizes[2];
    println!("ANSWER = {answer}");
}