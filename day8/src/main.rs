fn main() {
    soln2();
}

struct UnionFind {
    p: Vec<usize>,
    set_size: Vec<i64>,
    num_sets: usize
}

impl UnionFind {
    fn init(&mut self, n: usize) {
        self.p.resize(n, 0);
        self.set_size.resize(n, 1);

        for i in 0..n {
            self.p[i] = i;
        }

        self.num_sets = n;
    }

    fn find_set(&mut self, i: usize) -> usize {
        if self.p[i] == i {
            return i;
        }
        self.p[i] = self.find_set(self.p[i]);
        self.p[i]
    }

    fn union_set(&mut self, i: usize, j: usize) -> bool {
        let mut x = self.find_set(i);
        let mut y = self.find_set(j);

        if x == y {
            return false;
        }

        if self.set_size[x] < self.set_size[y] {
            let tmp = x;
            x = y;
            y = tmp;
        }
        self.set_size[y] += self.set_size[x];
        self.p[x] = y;

        self.num_sets -= 1;

        true
    }

    fn largest_three(&mut self) -> i64 {
        let mut v: Vec<i64> = Vec::new();
        for i in 0..self.p.len() {
            if self.p[i] == i {
                v.push(self.set_size[i]);
            }
        }

        v.sort_unstable();

        let mut prod = 1i64;
        for _ in 0..3 {
            prod *= v.pop().unwrap();
        }

        prod
    }
}

fn soln1() {
    use std::io;
    let stdin = io::stdin();

    let mut pts: Vec<(i64, i64, i64)> = Vec::new();
    loop {
        let mut buffer = String::new();
        let _ = stdin.read_line(&mut buffer);

        if buffer.len() < 3 {
            break;
        }

        let c = buffer.clone();
        let mut iter = c.trim().split(',').map(|v| v.parse::<i64>().unwrap());
        pts.push((iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()));
    }

    let mut dists: Vec<(i64, usize, usize)> = Vec::new();

    for i in 0..pts.len() {
        for j in 0..i {
            let dx = pts[i].0-pts[j].0;
            let dy = pts[i].1-pts[j].1;
            let dz = pts[i].2-pts[j].2;
            dists.push((dx*dx + dy*dy + dz*dz, i, j));
        }
    }

    dists.sort_unstable();

    let mut uf = UnionFind { p: Vec::new(), set_size: Vec::new(), num_sets: 0 };
    uf.init(pts.len());

    for idx in 0..1000 {
        uf.union_set(dists[idx].1, dists[idx].2);
    }

    println!("{}", uf.largest_three());
}

fn soln2() {
    use std::io;
    let stdin = io::stdin();

    let mut pts: Vec<(i64, i64, i64)> = Vec::new();
    loop {
        let mut buffer = String::new();
        let _ = stdin.read_line(&mut buffer);

        if buffer.len() < 3 {
            break;
        }

        let c = buffer.clone();
        let mut iter = c.trim().split(',').map(|v| v.parse::<i64>().unwrap());
        pts.push((iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap()));
    }

    let mut dists: Vec<(i64, usize, usize)> = Vec::new();

    for i in 0..pts.len() {
        for j in 0..i {
            let dx = pts[i].0-pts[j].0;
            let dy = pts[i].1-pts[j].1;
            let dz = pts[i].2-pts[j].2;
            dists.push((dx*dx + dy*dy + dz*dz, i, j));
        }
    }

    dists.sort_unstable();

    let mut uf = UnionFind { p: Vec::new(), set_size: Vec::new(), num_sets: 0 };
    uf.init(pts.len());

    let mut xs = 0;
    for idx in 0..dists.len() {
        if uf.union_set(dists[idx].1, dists[idx].2) {
            xs = pts[dists[idx].1].0 * pts[dists[idx].2].0;
        }
    }

    println!("{}", xs);
}
