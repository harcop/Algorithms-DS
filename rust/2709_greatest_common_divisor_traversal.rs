/// LeetCode #2709 - Greatest Common Divisor Traversal
use std::collections::HashSet;

struct UnionFind {
    p: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            p: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.p[x] != x {
            self.p[x] = self.find(self.p[x]);
        }
        self.p[x]
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
        self.p[pb] = pa;
        self.size[pa] += self.size[pb];
    }
}

fn prime_factors(mut v: i32) -> Vec<i32> {
    let mut factors = Vec::new();
    let mut i = 2;
    while i * i <= v {
        if v % i == 0 {
            factors.push(i);
            while v % i == 0 {
                v /= i;
            }
        }
        i += 1;
    }
    if v > 1 {
        factors.push(v);
    }
    factors
}

fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
    let n = nums.len();
    if n == 1 {
        return true;
    }
    let m = *nums.iter().max().unwrap() as usize;
    let mut uf = UnionFind::new(n + m + 1);
    for (i, &x) in nums.iter().enumerate() {
        for j in prime_factors(x) {
            uf.union(i, j as usize + n);
        }
    }
    let mut roots = HashSet::new();
    for i in 0..n {
        roots.insert(uf.find(i));
    }
    roots.len() == 1
}

fn main() {
    println!("{}", can_traverse_all_pairs(vec![2, 3, 6]));
}

#[cfg(test)]
mod tests {
    use super::can_traverse_all_pairs;

    #[test]
    fn example_one() {
        assert!(can_traverse_all_pairs(vec![2, 3, 6]));
    }

    #[test]
    fn example_two() {
        assert!(!can_traverse_all_pairs(vec![3, 9, 5]));
    }

    #[test]
    fn example_three() {
        assert!(can_traverse_all_pairs(vec![4, 3, 12, 8]));
    }
}
