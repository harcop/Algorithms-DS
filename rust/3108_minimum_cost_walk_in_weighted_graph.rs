/// LeetCode #3108 - Minimum Cost Walk in Weighted Graph
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

    fn union(&mut self, a: usize, b: usize) {
        let mut pa = self.find(a);
        let mut pb = self.find(b);
        if pa == pb {
            return;
        }
        if self.size[pa] < self.size[pb] {
            std::mem::swap(&mut pa, &mut pb);
        }
        self.parent[pb] = pa;
        self.size[pa] += self.size[pb];
    }
}

fn minimum_cost(n: i32, edges: Vec<Vec<i32>>, query: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let mut uf = UnionFind::new(n);
    for e in &edges {
        uf.union(e[0] as usize, e[1] as usize);
    }
    let mut g = vec![-1i32; n];
    for e in &edges {
        let root = uf.find(e[0] as usize);
        g[root] &= e[2];
    }
    query
        .into_iter()
        .map(|q| {
            let (u, v) = (q[0] as usize, q[1] as usize);
            if u == v {
                return 0;
            }
            let a = uf.find(u);
            let b = uf.find(v);
            if a == b {
                g[a]
            } else {
                -1
            }
        })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        minimum_cost(
            5,
            vec![vec![0, 1, 7], vec![1, 3, 7], vec![1, 2, 1]],
            vec![vec![0, 3], vec![3, 4]]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_cost;

    #[test]
    fn example1() {
        assert_eq!(
            minimum_cost(
                5,
                vec![vec![0, 1, 7], vec![1, 3, 7], vec![1, 2, 1]],
                vec![vec![0, 3], vec![3, 4]]
            ),
            vec![1, -1]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            minimum_cost(
                3,
                vec![vec![0, 2, 7], vec![0, 1, 15], vec![1, 2, 6], vec![1, 2, 1]],
                vec![vec![1, 2]]
            ),
            vec![0]
        );
    }
}
