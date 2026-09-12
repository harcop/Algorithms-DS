/// LeetCode #3721 - Longest Balanced Subarray II
use std::collections::HashMap;

struct SegmentTree {
    minv: Vec<i32>,
    maxv: Vec<i32>,
    lazy: Vec<i32>,
    base: usize,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        let mut size = 1;
        while size < n {
            size <<= 1;
        }
        Self {
            minv: vec![0; size << 1],
            maxv: vec![0; size << 1],
            lazy: vec![0; size],
            base: size,
        }
    }

    fn apply(&mut self, x: usize, val: i32) {
        self.minv[x] += val;
        self.maxv[x] += val;
        if x < self.base {
            self.lazy[x] += val;
        }
    }

    fn pull(&mut self, mut x: usize) {
        while x > 1 {
            x >>= 1;
            self.minv[x] = self.minv[x << 1].min(self.minv[(x << 1) | 1]);
            self.maxv[x] = self.maxv[x << 1].max(self.maxv[(x << 1) | 1]);
            if self.lazy[x] != 0 {
                self.minv[x] += self.lazy[x];
                self.maxv[x] += self.lazy[x];
            }
        }
    }

    fn update(&mut self, mut l: usize, mut r: usize, h: i32) {
        l += self.base;
        r += self.base;
        let (l0, r0) = (l, r);
        while l <= r {
            if l & 1 == 1 {
                self.apply(l, h);
                l += 1;
            }
            if r & 1 == 0 {
                self.apply(r, h);
                r -= 1;
            }
            l >>= 1;
            r >>= 1;
        }
        self.pull(l0);
        self.pull(r0);
    }

    fn binary_search(&mut self, x: i32) -> usize {
        let mut i = 1usize;
        while i < self.base {
            if self.lazy[i] != 0 {
                let v = self.lazy[i];
                self.apply(i << 1, v);
                self.apply((i << 1) | 1, v);
                self.lazy[i] = 0;
            }
            i <<= 1;
            if !(self.minv[i] <= x && x <= self.maxv[i]) {
                i |= 1;
            }
        }
        i - self.base
    }
}

fn longest_balanced(nums: Vec<i32>) -> i32 {
    let n = nums.len() + 1;
    let mut st = SegmentTree::new(n);
    let mut result = 0i32;
    let mut curr = 0i32;
    let mut lookup: HashMap<i32, usize> = HashMap::new();
    for (idx, x) in nums.into_iter().enumerate() {
        let i = idx + 1;
        let d = if x & 1 == 1 { 1 } else { -1 };
        if let Some(&prev) = lookup.get(&x) {
            st.update(prev, n - 1, -d);
            curr -= d;
        }
        curr += d;
        lookup.insert(x, i);
        st.update(i, n - 1, d);
        let l = i as i32 - st.binary_search(curr) as i32;
        if l > result {
            result = l;
        }
    }
    result
}

fn main() {
    println!("{}", longest_balanced(vec![2, 5, 4, 3]));
}

#[cfg(test)]
mod tests {
    use super::longest_balanced;

    #[test]
    fn example1() {
        assert_eq!(longest_balanced(vec![2, 5, 4, 3]), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(longest_balanced(vec![3, 2, 2, 5, 4]), 5);
    }

    #[test]
    fn example3() {
        assert_eq!(longest_balanced(vec![1, 2, 3, 2]), 3);
    }
}
