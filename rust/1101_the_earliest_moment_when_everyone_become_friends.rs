/// LeetCode #1101 - The Earliest Moment When Everyone Become Friends
struct UnionFind {
    parent: Vec<usize>,
    size: usize,
    count: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: n,
            count: n,
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let p = self.find(self.parent[x]);
            self.parent[x] = p;
        }
        self.parent[x]
    }
    fn union(&mut self, a: usize, b: usize) -> bool {
        let (mut a, mut b) = (self.find(a), self.find(b));
        if a == b {
            return false;
        }
        if a < b {
            std::mem::swap(&mut a, &mut b);
        }
        self.parent[b] = a;
        self.count -= 1;
        self.count == 1
    }
}

fn earliest_acq(logs: Vec<Vec<i32>>, n: i32) -> i32 {
    let n = n as usize;
    let mut logs = logs;
    logs.sort_by_key(|x| x[0]);
    let mut uf = UnionFind::new(n);
    for e in logs {
        if uf.union(e[1] as usize, e[2] as usize) {
            return e[0];
        }
    }
    -1
}

fn main() {
    println!(
        "{}",
        earliest_acq(vec![vec![0, 2, 0], vec![1, 0, 1], vec![3, 0, 3], vec![4, 1, 2], vec![7, 3, 3]], 4)
    );
}

#[cfg(test)]
mod tests {
    use super::earliest_acq;

    #[test]
    fn example_one() {
        assert_eq!(
            earliest_acq(
                vec![vec![0, 2, 0], vec![1, 0, 1], vec![3, 0, 3], vec![4, 1, 2], vec![7, 3, 3]],
                4
            ),
            3
        );
    }
}
