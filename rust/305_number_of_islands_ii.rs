/// LeetCode #305 - Number of Islands II
struct DSU {
    p: Vec<usize>,
    ncomp: i32,
}

impl DSU {
    fn new(n: usize) -> Self {
        DSU {
            p: (0..n).collect(),
            ncomp: 0,
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.p[x] != x {
            self.p[x] = self.find(self.p[x]);
        }
        self.p[x]
    }
    fn union(&mut self, a: usize, b: usize) -> bool {
        let a = self.find(a);
        let b = self.find(b);
        if a == b {
            return false;
        }
        self.p[b] = a;
        true
    }
}

fn num_islands2(m: i32, n: i32, positions: Vec<Vec<i32>>) -> Vec<i32> {
    let m = m as usize;
    let n = n as usize;
    let sz = m * n;
    let mut grid = vec![false; sz];
    let mut dsu = DSU::new(sz);
    let mut out = vec![];
    for pos in positions {
        let r = pos[0] as usize;
        let c = pos[1] as usize;
        let id = r * n + c;
        if grid[id] {
            out.push(dsu.ncomp);
            continue;
        }
        grid[id] = true;
        dsu.ncomp += 1;
        for (dr, dc) in [(0i32, 1i32), (0, -1), (1, 0), (-1, 0)] {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if nr >= 0 && nr < m as i32 && nc >= 0 && nc < n as i32 {
                let nid = nr as usize * n + nc as usize;
                if grid[nid] && dsu.union(id, nid) {
                    dsu.ncomp -= 1;
                }
            }
        }
        out.push(dsu.ncomp);
    }
    out
}

fn main() {
    println!(
        "{:?}",
        num_islands2(3, 3, vec![vec![0, 0], vec![0, 1], vec![1, 2], vec![2, 1]])
    );
}

#[cfg(test)]
mod tests {
    use super::num_islands2;

    #[test]
    fn example_one() {
        assert_eq!(
            num_islands2(3, 3, vec![vec![0, 0], vec![0, 1], vec![1, 2], vec![2, 1]]),
            vec![1, 1, 2, 3]
        );
    }
}
