/// LeetCode #2407 - Longest Increasing Subsequence II
struct SegTree {
    n: usize,
    tree: Vec<i32>,
}

impl SegTree {
    fn new(size: usize) -> Self {
        Self {
            n: size,
            tree: vec![0; size * 4],
        }
    }

    fn update(&mut self, idx: usize, val: i32, node: usize, left: usize, right: usize) {
        if left == right {
            self.tree[node] = self.tree[node].max(val);
            return;
        }
        let mid = (left + right) / 2;
        if idx <= mid {
            self.update(idx, val, node * 2, left, mid);
        } else {
            self.update(idx, val, node * 2 + 1, mid + 1, right);
        }
        self.tree[node] = self.tree[node * 2].max(self.tree[node * 2 + 1]);
    }

    fn query(&self, ql: usize, qr: usize, node: usize, left: usize, right: usize) -> i32 {
        if ql > right || qr < left {
            return 0;
        }
        if ql <= left && right <= qr {
            return self.tree[node];
        }
        let mid = (left + right) / 2;
        self.query(ql, qr, node * 2, left, mid)
            .max(self.query(ql, qr, node * 2 + 1, mid + 1, right))
    }
}

fn length_of_lis(nums: Vec<i32>, k: i32) -> i32 {
    let max_val = *nums.iter().max().unwrap() as usize;
    let mut seg = SegTree::new(max_val.max(1));
    let mut ans = 0;

    for num in nums {
        let num = num as usize;
        let left = num.saturating_sub(k as usize);
        let best = if left <= num.saturating_sub(1) && num > 1 {
            seg.query(left.max(1) - 1, num - 2, 1, 0, seg.n - 1)
        } else {
            0
        };
        let cur = best + 1;
        seg.update(num - 1, cur, 1, 0, seg.n - 1);
        ans = ans.max(cur);
    }

    ans
}

fn main() {
    println!("{}", length_of_lis(vec![4, 2, 1, 4, 3, 4, 5, 8, 15], 3));
}

#[cfg(test)]
mod tests {
    use super::length_of_lis;

    #[test]
    fn example_one() {
        assert_eq!(length_of_lis(vec![4, 2, 1, 4, 3, 4, 5, 8, 15], 3), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(length_of_lis(vec![7, 4, 5, 1, 8, 12, 4, 7], 5), 4);
    }
}
