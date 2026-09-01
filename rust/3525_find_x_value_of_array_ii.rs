/// LeetCode #3525 - Find X Value of Array II
#[derive(Clone)]
struct Node {
    prod: usize,
    cnt: [i32; 5],
}

impl Node {
    fn identity() -> Self {
        Self {
            prod: 1,
            cnt: [0; 5],
        }
    }

    fn leaf(val: i32, k: usize) -> Self {
        let m = (val as usize) % k;
        let mut cnt = [0; 5];
        cnt[m] = 1;
        Self { prod: m, cnt }
    }
}

fn merge(left: &Node, right: &Node, k: usize) -> Node {
    let mut cnt = [0; 5];
    for r in 0..k {
        cnt[r] = left.cnt[r];
    }
    for r in 0..k {
        if right.cnt[r] != 0 {
            cnt[(left.prod * r) % k] += right.cnt[r];
        }
    }
    Node {
        prod: (left.prod * right.prod) % k,
        cnt,
    }
}

struct SegTree {
    n: usize,
    k: usize,
    t: Vec<Node>,
}

impl SegTree {
    fn new(nums: &[i32], k: usize) -> Self {
        let n = nums.len();
        let mut st = Self {
            n,
            k,
            t: vec![Node::identity(); n * 4],
        };
        st.build(1, 0, n - 1, nums);
        st
    }

    fn build(&mut self, p: usize, l: usize, r: usize, nums: &[i32]) {
        if l == r {
            self.t[p] = Node::leaf(nums[l], self.k);
            return;
        }
        let m = (l + r) / 2;
        self.build(p * 2, l, m, nums);
        self.build(p * 2 + 1, m + 1, r, nums);
        self.t[p] = merge(&self.t[p * 2], &self.t[p * 2 + 1], self.k);
    }

    fn update(&mut self, p: usize, l: usize, r: usize, idx: usize, val: i32) {
        if l == r {
            self.t[p] = Node::leaf(val, self.k);
            return;
        }
        let m = (l + r) / 2;
        if idx <= m {
            self.update(p * 2, l, m, idx, val);
        } else {
            self.update(p * 2 + 1, m + 1, r, idx, val);
        }
        self.t[p] = merge(&self.t[p * 2], &self.t[p * 2 + 1], self.k);
    }

    fn query(&self, p: usize, l: usize, r: usize, ql: usize, qr: usize) -> Node {
        if ql > r || qr < l {
            return Node::identity();
        }
        if ql <= l && r <= qr {
            return self.t[p].clone();
        }
        let m = (l + r) / 2;
        let left = self.query(p * 2, l, m, ql, qr);
        let right = self.query(p * 2 + 1, m + 1, r, ql, qr);
        merge(&left, &right, self.k)
    }
}

fn result_array(nums: Vec<i32>, k: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = nums.len();
    let k = k as usize;
    let mut st = SegTree::new(&nums, k);
    queries
        .into_iter()
        .map(|q| {
            let index = q[0] as usize;
            let value = q[1];
            let start = q[2] as usize;
            let x = q[3] as usize;
            st.update(1, 0, n - 1, index, value);
            let node = st.query(1, 0, n - 1, start, n - 1);
            node.cnt[x]
        })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        result_array(
            vec![1, 2, 3, 4, 5],
            3,
            vec![vec![2, 2, 0, 2], vec![3, 3, 3, 0], vec![0, 1, 0, 1]]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::result_array;

    #[test]
    fn example1() {
        assert_eq!(
            result_array(
                vec![1, 2, 3, 4, 5],
                3,
                vec![vec![2, 2, 0, 2], vec![3, 3, 3, 0], vec![0, 1, 0, 1]]
            ),
            vec![2, 2, 2]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            result_array(
                vec![1, 2, 4, 8, 16, 32],
                4,
                vec![vec![0, 2, 0, 2], vec![0, 2, 0, 1]]
            ),
            vec![1, 0]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            result_array(vec![1, 1, 2, 1, 1], 2, vec![vec![2, 1, 0, 1]]),
            vec![5]
        );
    }
}
