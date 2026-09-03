/// LeetCode #3569 - Maximize Count of Distinct Primes After Split
use std::collections::{BTreeSet, HashMap};

struct SegTree {
    n: usize,
    tree: Vec<i32>,
    lazy: Vec<i32>,
}

impl SegTree {
    fn new(n: usize) -> Self {
        Self {
            n,
            tree: vec![0; 4 * n.max(1)],
            lazy: vec![0; 4 * n.max(1)],
        }
    }
    fn push(&mut self, i: usize) {
        if self.lazy[i] != 0 {
            let v = self.lazy[i];
            for c in [i * 2, i * 2 + 1] {
                self.tree[c] += v;
                self.lazy[c] += v;
            }
            self.lazy[i] = 0;
        }
    }
    fn update(&mut self, i: usize, l: usize, r: usize, ql: usize, qr: usize, v: i32) {
        if ql > r || qr < l {
            return;
        }
        if ql <= l && r <= qr {
            self.tree[i] += v;
            self.lazy[i] += v;
            return;
        }
        self.push(i);
        let mid = (l + r) / 2;
        self.update(i * 2, l, mid, ql, qr, v);
        self.update(i * 2 + 1, mid + 1, r, ql, qr, v);
        self.tree[i] = self.tree[i * 2].max(self.tree[i * 2 + 1]);
    }
    fn range_add(&mut self, l: usize, r: usize, v: i32) {
        if self.n == 0 || l > r {
            return;
        }
        self.update(1, 0, self.n - 1, l, r, v);
    }
    fn query_max(&self) -> i32 {
        if self.n == 0 {
            0
        } else {
            self.tree[1]
        }
    }
}

fn sieve(n: usize) -> Vec<bool> {
    let mut is_prime = vec![true; n + 1];
    if n >= 0 {
        is_prime[0] = false;
    }
    if n >= 1 {
        is_prime[1] = false;
    }
    let mut i = 2usize;
    while i * i <= n {
        if is_prime[i] {
            let mut j = i * i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    is_prime
}

fn maximum_count(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = nums.len();
    let is_prime = sieve(100_000);
    let mut lookup: HashMap<i32, BTreeSet<usize>> = HashMap::new();
    let mut st = SegTree::new(n.saturating_sub(1));

    let mut add = |nums: &[i32], lookup: &mut HashMap<i32, BTreeSet<usize>>, st: &mut SegTree, i: usize, d: i32| {
        let x = nums[i];
        if x as usize > 100_000 || !is_prime[x as usize] {
            return;
        }
        if d == 1 {
            lookup.entry(x).or_default().insert(i);
        }
        let set = lookup.get(&x).unwrap();
        if set.len() == 1 {
            if n >= 2 {
                st.range_add(0, n - 2, d);
            }
        } else {
            let first = *set.iter().next().unwrap();
            let second = *set.iter().nth(1).unwrap();
            let last = *set.iter().next_back().unwrap();
            let second_last = *set.iter().rev().nth(1).unwrap();
            if i == first {
                st.range_add(i, second - 1, d);
            } else if i == last {
                st.range_add(second_last, i - 1, d);
            }
        }
        if d == -1 {
            lookup.get_mut(&x).unwrap().remove(&i);
            if lookup.get(&x).unwrap().is_empty() {
                lookup.remove(&x);
            }
        }
    };

    for i in 0..n {
        add(&nums, &mut lookup, &mut st, i, 1);
    }
    let mut ans = Vec::with_capacity(queries.len());
    for q in queries {
        let idx = q[0] as usize;
        let x = q[1];
        if nums[idx] != x {
            add(&nums, &mut lookup, &mut st, idx, -1);
            nums[idx] = x;
            add(&nums, &mut lookup, &mut st, idx, 1);
        }
        ans.push(st.query_max());
    }
    ans
}

fn main() {
    println!("{:?}", maximum_count(vec![2, 1, 3, 1, 2], vec![vec![1, 2], vec![3, 3]]));
}

#[cfg(test)]
mod tests {
    use super::maximum_count;

    #[test]
    fn example1() {
        assert_eq!(
            maximum_count(vec![2, 1, 3, 1, 2], vec![vec![1, 2], vec![3, 3]]),
            vec![3, 4]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(maximum_count(vec![2, 1, 4], vec![vec![0, 1]]), vec![0]);
    }
}
