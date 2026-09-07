/// LeetCode #3607 - Power Grid Maintenance
use std::collections::BTreeSet;

struct UnionFind {
    p: Vec<i32>,
    sz: Vec<i32>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            p: (0..n as i32).collect(),
            sz: vec![1; n],
        }
    }
    fn find(&mut self, x: i32) -> i32 {
        let i = x as usize;
        if self.p[i] != x {
            self.p[i] = self.find(self.p[i]);
        }
        self.p[i]
    }
    fn union(&mut self, a: i32, b: i32) {
        let (pa, pb) = (self.find(a), self.find(b));
        if pa == pb {
            return;
        }
        let (a, b) = (pa as usize, pb as usize);
        if self.sz[a] < self.sz[b] {
            self.p[a] = pb;
            self.sz[b] += self.sz[a];
        } else {
            self.p[b] = pa;
            self.sz[a] += self.sz[b];
        }
    }
}

fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let c = c as usize;
    let mut uf = UnionFind::new(c + 1);
    for e in &connections {
        uf.union(e[0], e[1]);
    }
    let mut st = vec![BTreeSet::new(); c + 1];
    for i in 1..=c {
        let root = uf.find(i as i32) as usize;
        st[root].insert(i as i32);
    }
    let mut ans = Vec::new();
    for q in queries {
        let (a, x) = (q[0], q[1]);
        let root = uf.find(x) as usize;
        if a == 1 {
            if st[root].contains(&x) {
                ans.push(x);
            } else if let Some(&y) = st[root].iter().next() {
                ans.push(y);
            } else {
                ans.push(-1);
            }
        } else {
            st[root].remove(&x);
        }
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        process_queries(
            5,
            vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]],
            vec![vec![1, 3], vec![2, 1], vec![1, 1], vec![2, 2], vec![1, 2]]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::process_queries;

    #[test]
    fn example1() {
        assert_eq!(
            process_queries(
                5,
                vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]],
                vec![vec![1, 3], vec![2, 1], vec![1, 1], vec![2, 2], vec![1, 2]]
            ),
            vec![3, 2, 3]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            process_queries(3, vec![], vec![vec![1, 1], vec![2, 1], vec![1, 1]]),
            vec![1, -1]
        );
    }
}
