/// LeetCode #3624 - Number of Integers With Popcount-Depth Equal to K II
fn popcount_depth(mut x: i64) -> i32 {
    let mut d = 0;
    while x > 1 {
        x = x.count_ones() as i64;
        d += 1;
    }
    d
}

struct SegTree {
    n: usize,
    tree: Vec<[i32; 6]>,
}

impl SegTree {
    fn new(depth: &[i32]) -> Self {
        let n = depth.len();
        let mut st = SegTree {
            n,
            tree: vec![[0; 6]; 4 * n.max(1)],
        };
        if n > 0 {
            st.build(1, 0, n - 1, depth);
        }
        st
    }

    fn build(&mut self, node: usize, l: usize, r: usize, depth: &[i32]) {
        if l == r {
            let d = depth[l] as usize;
            if d < 6 {
                self.tree[node][d] = 1;
            }
            return;
        }
        let mid = (l + r) / 2;
        self.build(node * 2, l, mid, depth);
        self.build(node * 2 + 1, mid + 1, r, depth);
        for i in 0..6 {
            self.tree[node][i] = self.tree[node * 2][i] + self.tree[node * 2 + 1][i];
        }
    }

    fn update(&mut self, node: usize, l: usize, r: usize, idx: usize, old_d: i32, new_d: i32) {
        if old_d >= 0 && (old_d as usize) < 6 {
            self.tree[node][old_d as usize] -= 1;
        }
        if new_d >= 0 && (new_d as usize) < 6 {
            self.tree[node][new_d as usize] += 1;
        }
        if l == r {
            return;
        }
        let mid = (l + r) / 2;
        if idx <= mid {
            self.update(node * 2, l, mid, idx, old_d, new_d);
        } else {
            self.update(node * 2 + 1, mid + 1, r, idx, old_d, new_d);
        }
    }

    fn query(&self, node: usize, l: usize, r: usize, ql: usize, qr: usize, k: usize) -> i32 {
        if qr < l || r < ql {
            return 0;
        }
        if ql <= l && r <= qr {
            return self.tree[node][k];
        }
        let mid = (l + r) / 2;
        self.query(node * 2, l, mid, ql, qr, k) + self.query(node * 2 + 1, mid + 1, r, ql, qr, k)
    }
}

fn popcount_depth_ii(mut nums: Vec<i64>, queries: Vec<Vec<i64>>) -> Vec<i32> {
    let n = nums.len();
    let depth: Vec<i32> = nums.iter().map(|&x| popcount_depth(x)).collect();
    let mut st = SegTree::new(&depth);
    let mut depths = depth;
    let mut ans = Vec::new();
    for q in queries {
        if q[0] == 1 {
            let l = q[1] as usize;
            let r = q[2] as usize;
            let k = q[3] as usize;
            ans.push(if n == 0 { 0 } else { st.query(1, 0, n - 1, l, r, k) });
        } else {
            let idx = q[1] as usize;
            let val = q[2];
            let old_d = depths[idx];
            let new_d = popcount_depth(val);
            nums[idx] = val;
            depths[idx] = new_d;
            if n > 0 {
                st.update(1, 0, n - 1, idx, old_d, new_d);
            }
        }
    }
    ans
}

fn main() {
    println!("{:?}", popcount_depth_ii(vec![2, 4], vec![vec![1, 0, 1, 1], vec![2, 1, 1], vec![1, 0, 1, 0]]));
}

#[cfg(test)]
mod tests {
    use super::popcount_depth_ii;

    #[test]
    fn example1() {
        assert_eq!(
            popcount_depth_ii(vec![2, 4], vec![vec![1, 0, 1, 1], vec![2, 1, 1], vec![1, 0, 1, 0]]),
            vec![2, 1]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            popcount_depth_ii(
                vec![3, 5, 6],
                vec![vec![1, 0, 2, 2], vec![2, 1, 4], vec![1, 1, 2, 1], vec![1, 0, 1, 0]]
            ),
            vec![3, 1, 0]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            popcount_depth_ii(
                vec![1, 2],
                vec![vec![1, 0, 1, 1], vec![2, 0, 3], vec![1, 0, 0, 1], vec![1, 0, 0, 2]]
            ),
            vec![1, 0, 1]
        );
    }
}
