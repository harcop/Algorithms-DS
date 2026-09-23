/// LeetCode #3901 - Good Subsequence Queries
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a.abs()
}

struct SegTree {
    n: usize,
    tree: Vec<i32>,
    nums: Vec<i32>,
}

impl SegTree {
    fn new(nums: Vec<i32>, p: i32) -> Self {
        let n = nums.len();
        let mut st = Self {
            n,
            tree: vec![0; n.max(1) * 4],
            nums: nums.clone(),
        };
        if n > 0 {
            st.build(1, 0, n - 1, &nums, p);
        }
        st
    }

    fn leaf(&self, v: i32, p: i32) -> i32 {
        if v % p == 0 { v } else { 0 }
    }

    fn build(&mut self, node: usize, l: usize, r: usize, nums: &[i32], p: i32) {
        if l == r {
            self.tree[node] = self.leaf(nums[l], p);
            return;
        }
        let m = (l + r) / 2;
        self.build(node * 2, l, m, nums, p);
        self.build(node * 2 + 1, m + 1, r, nums, p);
        self.tree[node] = gcd(self.tree[node * 2], self.tree[node * 2 + 1]);
    }

    fn update(&mut self, node: usize, l: usize, r: usize, idx: usize, val: i32, p: i32) {
        if l == r {
            self.nums[l] = val;
            self.tree[node] = self.leaf(val, p);
            return;
        }
        let m = (l + r) / 2;
        if idx <= m {
            self.update(node * 2, l, m, idx, val, p);
        } else {
            self.update(node * 2 + 1, m + 1, r, idx, val, p);
        }
        self.tree[node] = gcd(self.tree[node * 2], self.tree[node * 2 + 1]);
    }

    fn query(&self, node: usize, l: usize, r: usize, ql: usize, qr: usize) -> i32 {
        if ql > r || qr < l {
            return 0;
        }
        if ql <= l && r <= qr {
            return self.tree[node];
        }
        let m = (l + r) / 2;
        gcd(
            self.query(node * 2, l, m, ql, qr),
            self.query(node * 2 + 1, m + 1, r, ql, qr),
        )
    }

    fn root_gcd(&self) -> i32 {
        if self.n == 0 {
            0
        } else {
            self.query(1, 0, self.n - 1, 0, self.n - 1)
        }
    }

    fn cnt_divisible(&self, p: i32) -> usize {
        self.nums.iter().filter(|&&x| x % p == 0).count()
    }

    fn has_good(&self, p: i32) -> bool {
        let n = self.n;
        let g = self.root_gcd();
        if g != p {
            return false;
        }
        let cnt = self.cnt_divisible(p);
        if cnt < n || n > 6 {
            return true;
        }
        for i in 0..n {
            let left = if i == 0 {
                0
            } else {
                self.query(1, 0, n - 1, 0, i - 1)
            };
            let right = if i + 1 >= n {
                0
            } else {
                self.query(1, 0, n - 1, i + 1, n - 1)
            };
            if gcd(left, right) == p {
                return true;
            }
        }
        false
    }
}

fn good_subsequence_queries(nums: Vec<i32>, p: i32, queries: Vec<Vec<i32>>) -> i32 {
    let mut st = SegTree::new(nums, p);
    let mut ans = 0;
    for q in queries {
        let ind = q[0] as usize;
        let val = q[1];
        st.update(1, 0, st.n - 1, ind, val, p);
        if st.has_good(p) {
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        good_subsequence_queries(vec![4, 8, 12, 16], 2, vec![vec![0, 3], vec![2, 6]])
    );
}

#[cfg(test)]
mod tests {
    use super::good_subsequence_queries;

    #[test]
    fn example1() {
        assert_eq!(
            good_subsequence_queries(vec![4, 8, 12, 16], 2, vec![vec![0, 3], vec![2, 6]]),
            1
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            good_subsequence_queries(vec![4, 5, 7, 8], 3, vec![vec![0, 6], vec![1, 9], vec![2, 3]]),
            2
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            good_subsequence_queries(vec![5, 7, 9], 2, vec![vec![1, 4], vec![2, 8]]),
            0
        );
    }
}
