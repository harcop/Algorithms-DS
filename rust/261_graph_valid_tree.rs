/// LeetCode #261 - Graph Valid Tree
struct DSU {
    p: Vec<usize>,
    r: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        DSU {
            p: (0..n).collect(),
            r: vec![0; n],
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.p[x] != x {
            self.p[x] = self.find(self.p[x]);
        }
        self.p[x]
    }
    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut a = self.find(a);
        let mut b = self.find(b);
        if a == b {
            return false;
        }
        if self.r[a] < self.r[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.p[b] = a;
        if self.r[a] == self.r[b] {
            self.r[a] += 1;
        }
        true
    }
}

fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
    let n = n as usize;
    if edges.len() != n - 1 {
        return false;
    }
    let mut dsu = DSU::new(n);
    for e in edges {
        if !dsu.union(e[0] as usize, e[1] as usize) {
            return false;
        }
    }
    (1..n).all(|i| dsu.find(i) == dsu.find(0))
}

fn main() {
    println!("{}", valid_tree(5, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4]]));
}

#[cfg(test)]
mod tests {
    use super::valid_tree;

    #[test]
    fn example_one() {
        assert!(valid_tree(
            5,
            vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4]]
        ));
    }

    #[test]
    fn example_two() {
        assert!(!valid_tree(
            5,
            vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![1, 3], vec![1, 4]]
        ));
    }
}
