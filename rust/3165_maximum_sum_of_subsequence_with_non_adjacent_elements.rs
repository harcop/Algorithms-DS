/// LeetCode #3165 - Maximum Sum of Subsequence With Non-adjacent Elements
const MOD: i64 = 1_000_000_007;

#[derive(Clone, Copy, Default)]
struct Node {
    s00: i64,
    s01: i64,
    s10: i64,
    s11: i64,
}

fn merge(left: Node, right: Node) -> Node {
    Node {
        s00: (left.s00 + right.s10).max(left.s01 + right.s00),
        s01: (left.s00 + right.s11).max(left.s01 + right.s01),
        s10: (left.s10 + right.s10).max(left.s11 + right.s00),
        s11: (left.s10 + right.s11).max(left.s11 + right.s01),
    }
}

struct SegTree {
    n: usize,
    tree: Vec<Node>,
}

impl SegTree {
    fn new(n: usize) -> Self {
        let mut st = Self {
            n,
            tree: vec![Node::default(); n * 4 + 4],
        };
        st.build(1, 1, n);
        st
    }

    fn build(&mut self, u: usize, l: usize, r: usize) {
        if l == r {
            return;
        }
        let mid = (l + r) / 2;
        self.build(u * 2, l, mid);
        self.build(u * 2 + 1, mid + 1, r);
    }

    fn modify(&mut self, u: usize, l: usize, r: usize, x: usize, v: i64) {
        if l == r {
            self.tree[u].s11 = v.max(0);
            self.tree[u].s00 = 0;
            self.tree[u].s01 = 0;
            self.tree[u].s10 = 0;
            return;
        }
        let mid = (l + r) / 2;
        if x <= mid {
            self.modify(u * 2, l, mid, x, v);
        } else {
            self.modify(u * 2 + 1, mid + 1, r, x, v);
        }
        self.tree[u] = merge(self.tree[u * 2], self.tree[u * 2 + 1]);
    }

    fn set(&mut self, x: usize, v: i64) {
        self.modify(1, 1, self.n, x, v);
    }

    fn query_all(&self) -> i64 {
        let t = self.tree[1];
        t.s00.max(t.s01).max(t.s10).max(t.s11).max(0)
    }
}

fn maximum_sum_subsequence(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
    let n = nums.len();
    let mut tree = SegTree::new(n);
    for (i, &x) in nums.iter().enumerate() {
        tree.set(i + 1, x as i64);
    }
    let mut ans = 0i64;
    for q in queries {
        tree.set(q[0] as usize + 1, q[1] as i64);
        ans = (ans + tree.query_all()) % MOD;
    }
    ans as i32
}

fn main() {
    println!(
        "{}",
        maximum_sum_subsequence(vec![3, 5, 9], vec![vec![1, -2], vec![0, -3]])
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_sum_subsequence;

    #[test]
    fn example1() {
        assert_eq!(
            maximum_sum_subsequence(vec![3, 5, 9], vec![vec![1, -2], vec![0, -3]]),
            21
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            maximum_sum_subsequence(vec![0, -1], vec![vec![0, -5]]),
            0
        );
    }
}
