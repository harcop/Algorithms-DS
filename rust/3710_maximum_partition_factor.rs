/// LeetCode #3710 - Maximum Partition Factor
fn max_partition_factor(points: Vec<Vec<i32>>) -> i32 {
    struct UF {
        parent: Vec<usize>,
        rank: Vec<i32>,
        parity: Vec<i32>,
    }
    impl UF {
        fn new(n: usize) -> Self {
            Self {
                parent: (0..n).collect(),
                rank: vec![0; n],
                parity: vec![0; n],
            }
        }
        fn find(&mut self, mut x: usize) -> usize {
            let mut stk = Vec::new();
            while self.parent[x] != x {
                stk.push(x);
                x = self.parent[x];
            }
            while let Some(y) = stk.pop() {
                self.parity[y] ^= self.parity[self.parent[y]];
                self.parent[y] = x;
            }
            x
        }
        /// Returns false on conflict (same component, same parity).
        fn union(&mut self, ox: usize, oy: usize) -> bool {
            let mut x = self.find(ox);
            let mut y = self.find(oy);
            let mut ox = ox;
            let mut oy = oy;
            if x == y {
                return self.parity[ox] != self.parity[oy];
            }
            if self.rank[x] > self.rank[y] {
                std::mem::swap(&mut x, &mut y);
                std::mem::swap(&mut ox, &mut oy);
            }
            if self.rank[x] == self.rank[y] {
                self.rank[y] += 1;
            }
            self.parent[x] = y;
            self.parity[x] = self.parity[ox] ^ self.parity[oy] ^ 1;
            true
        }
    }

    let n = points.len();
    let mut dists = Vec::new();
    for u in 0..n {
        for v in u + 1..n {
            let d = (points[u][0] - points[v][0]).abs() + (points[u][1] - points[v][1]).abs();
            dists.push((d, u, v));
        }
    }
    dists.sort_unstable();
    let mut uf = UF::new(n);
    for (d, u, v) in dists {
        if !uf.union(u, v) {
            return d;
        }
    }
    0
}

fn main() {
    println!(
        "{}",
        max_partition_factor(vec![
            vec![0, 0],
            vec![0, 2],
            vec![2, 0],
            vec![2, 2]
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::max_partition_factor;

    #[test]
    fn example1() {
        assert_eq!(
            max_partition_factor(vec![
                vec![0, 0],
                vec![0, 2],
                vec![2, 0],
                vec![2, 2]
            ]),
            4
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            max_partition_factor(vec![vec![0, 0], vec![0, 1], vec![10, 0]]),
            11
        );
    }
}
