/// LeetCode #3873 - Maximum Points Activated with One Addition
use std::collections::HashMap;

struct UnionFind {
    p: HashMap<i64, i64>,
    size: HashMap<i64, i32>,
}

impl UnionFind {
    fn new() -> Self {
        Self {
            p: HashMap::new(),
            size: HashMap::new(),
        }
    }

    fn find(&mut self, x: i64) -> i64 {
        if !self.p.contains_key(&x) {
            self.p.insert(x, x);
            self.size.insert(x, 1);
        }
        let px = self.p[&x];
        if px != x {
            let root = self.find(px);
            self.p.insert(x, root);
            root
        } else {
            x
        }
    }

    fn union(&mut self, a: i64, b: i64) {
        let pa = self.find(a);
        let pb = self.find(b);
        if pa == pb {
            return;
        }
        let sa = self.size[&pa];
        let sb = self.size[&pb];
        if sa > sb {
            self.p.insert(pb, pa);
            self.size.insert(pa, sa + sb);
        } else {
            self.p.insert(pa, pb);
            self.size.insert(pb, sa + sb);
        }
    }
}

fn max_activated(points: Vec<Vec<i32>>) -> i32 {
    let mut uf = UnionFind::new();
    let m = 3_000_000_000i64;
    for p in &points {
        uf.union(p[0] as i64, p[1] as i64 + m);
    }
    let mut cnt: HashMap<i64, i32> = HashMap::new();
    for p in &points {
        let root = uf.find(p[0] as i64);
        *cnt.entry(root).or_insert(0) += 1;
    }
    let mut mx1 = 0;
    let mut mx2 = 0;
    for x in cnt.values().copied() {
        if mx1 < x {
            mx2 = mx1;
            mx1 = x;
        } else if mx2 < x {
            mx2 = x;
        }
    }
    mx1 + mx2 + 1
}

fn main() {
    println!(
        "{}",
        max_activated(vec![vec![1, 1], vec![1, 2], vec![2, 2]])
    );
}

#[cfg(test)]
mod tests {
    use super::max_activated;

    #[test]
    fn example1() {
        assert_eq!(
            max_activated(vec![vec![1, 1], vec![1, 2], vec![2, 2]]),
            4
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            max_activated(vec![vec![2, 2], vec![1, 1], vec![3, 3]]),
            3
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            max_activated(vec![vec![2, 3], vec![2, 2], vec![1, 1], vec![4, 5]]),
            4
        );
    }
}
