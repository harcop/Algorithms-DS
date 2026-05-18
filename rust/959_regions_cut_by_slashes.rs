/// LeetCode #959 - Regions Cut By Slashes

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
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
        if ra != rb {
            self.parent[rb] = ra;
        }
    }
}

fn regions_by_slashes(grid: Vec<String>) -> i32 {
    let n = grid.len();
    let mut uf = UnionFind::new(n * n * 4);
    let id = |r: usize, c: usize, d: usize| (r * n + c) * 4 + d;
    for r in 0..n {
        let row = grid[r].as_bytes();
        for c in 0..n {
            let ch = row[c];
            if ch == b' ' {
                uf.union(id(r, c, 0), id(r, c, 1));
                uf.union(id(r, c, 1), id(r, c, 2));
                uf.union(id(r, c, 2), id(r, c, 3));
            } else if ch == b'/' {
                uf.union(id(r, c, 0), id(r, c, 1));
                uf.union(id(r, c, 2), id(r, c, 3));
            } else {
                uf.union(id(r, c, 0), id(r, c, 3));
                uf.union(id(r, c, 1), id(r, c, 2));
            }
            if r + 1 < n {
                uf.union(id(r, c, 2), id(r + 1, c, 0));
            }
            if c + 1 < n {
                uf.union(id(r, c, 1), id(r, c + 1, 3));
            }
        }
    }
    let mut roots = std::collections::HashSet::new();
    for i in 0..n * n * 4 {
        roots.insert(uf.find(i));
    }
    roots.len() as i32
}

fn main() {
    println!("{}", regions_by_slashes(vec![" /".into(), "/ ".into()]));
}

#[cfg(test)]
mod tests {
    use super::regions_by_slashes;

    #[test]
    fn example_one() {
        assert_eq!(regions_by_slashes(vec![" /".into(), "/ ".into()]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(regions_by_slashes(vec![" /".into(), "  ".into()]), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(
            regions_by_slashes(vec!["//".into(), "/ ".into()]),
            2
        );
    }
}
