/// LeetCode #947 - Most Stones Removed with Same Row or Column
use std::collections::HashMap;

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
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
        let (ra, rb) = (self.find(a), self.find(b));
        if ra == rb {
            return;
        }
        if self.size[ra] < self.size[rb] {
            self.parent[ra] = rb;
            self.size[rb] += self.size[ra];
        } else {
            self.parent[rb] = ra;
            self.size[ra] += self.size[rb];
        }
    }
}

fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
    let n = stones.len();
    let mut uf = UnionFind::new(n);
    let mut row: HashMap<i32, usize> = HashMap::new();
    let mut col: HashMap<i32, usize> = HashMap::new();
    for (i, s) in stones.iter().enumerate() {
        let (r, c) = (s[0], s[1]);
        if let Some(&j) = row.get(&r) {
            uf.union(i, j);
        } else {
            row.insert(r, i);
        }
        if let Some(&j) = col.get(&c) {
            uf.union(i, j);
        } else {
            col.insert(c, i);
        }
    }
    let mut roots = 0usize;
    for i in 0..n {
        if uf.find(i) == i {
            roots += 1;
        }
    }
    (n - roots) as i32
}

fn main() {
    println!("{}", remove_stones(vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 2], vec![2, 1], vec![2, 2]]));
}

#[cfg(test)]
mod tests {
    use super::remove_stones;

    #[test]
    fn example_one() {
        assert_eq!(
            remove_stones(vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 2], vec![2, 1], vec![2, 2]]),
            5
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(remove_stones(vec![vec![0, 0], vec![0, 2], vec![1, 1], vec![2, 0], vec![2, 2]]), 3);
    }
}
