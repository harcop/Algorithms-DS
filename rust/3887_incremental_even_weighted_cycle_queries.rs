/// LeetCode #3887 - Incremental Even Weighted Cycle Queries
struct Dsu {
    parent: Vec<usize>,
    parity: Vec<i32>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Dsu {
            parent: (0..n).collect(),
            parity: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let p = self.parent[x];
            let root = self.find(p);
            self.parity[x] ^= self.parity[p];
            self.parent[x] = root;
        }
        self.parent[x]
    }

    fn try_add(&mut self, u: usize, v: usize, w: i32) -> bool {
        let w = w & 1;
        self.find(u);
        self.find(v);
        let ru = self.parent[u];
        let rv = self.parent[v];
        if ru == rv {
            return (self.parity[u] ^ self.parity[v] ^ w) == 0;
        }
        self.parent[rv] = ru;
        self.parity[rv] = self.parity[u] ^ self.parity[v] ^ w;
        true
    }
}

fn count_added_edges(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let mut dsu = Dsu::new(n as usize);
    let mut count = 0;
    for e in edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        let w = e[2];
        if dsu.try_add(u, v, w) {
            count += 1;
        }
    }
    count
}

fn main() {
    println!(
        "{}",
        count_added_edges(
            3,
            vec![vec![0, 1, 1], vec![1, 2, 1], vec![0, 2, 1]],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::count_added_edges;

    #[test]
    fn example1() {
        assert_eq!(
            count_added_edges(
                3,
                vec![vec![0, 1, 1], vec![1, 2, 1], vec![0, 2, 1]],
            ),
            2
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            count_added_edges(
                3,
                vec![vec![0, 1, 1], vec![1, 2, 1], vec![0, 2, 0]],
            ),
            3
        );
    }
}
