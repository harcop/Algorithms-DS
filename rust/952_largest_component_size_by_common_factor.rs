/// LeetCode #952 - Largest Component Size by Common Factor
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
    fn component_size(&mut self, x: usize) -> usize {
        let r = self.find(x);
        self.size[r]
    }
}

fn largest_component_size(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut uf = UnionFind::new(n);
    let mut factor_idx: HashMap<i32, usize> = HashMap::new();
    for (i, &x) in nums.iter().enumerate() {
        let mut num = x;
        let mut d = 2i32;
        while (d as i64) * (d as i64) <= num as i64 {
            if num % d == 0 {
                if let Some(&j) = factor_idx.get(&d) {
                    uf.union(i, j);
                } else {
                    factor_idx.insert(d, i);
                }
                while num % d == 0 {
                    num /= d;
                }
            }
            d += 1;
        }
        if num > 1 {
            if let Some(&j) = factor_idx.get(&num) {
                uf.union(i, j);
            } else {
                factor_idx.insert(num, i);
            }
        }
    }
    let mut best = 0usize;
    for i in 0..n {
        best = best.max(uf.component_size(i));
    }
    best as i32
}

fn main() {
    println!("{}", largest_component_size(vec![4, 6, 15, 35]));
}

#[cfg(test)]
mod tests {
    use super::largest_component_size;

    #[test]
    fn example_one() {
        assert_eq!(largest_component_size(vec![4, 6, 15, 35]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(largest_component_size(vec![20, 50, 9, 63]), 2);
    }
}
