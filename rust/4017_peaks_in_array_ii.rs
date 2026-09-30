/// LeetCode #4017 - Peaks in Array II
#[derive(Clone, Copy)]
struct Node {
    cnt: i32,
    first: i32,
    last: i32,
    gap: i64,
}

fn empty() -> Node {
    Node {
        cnt: 0,
        first: -1,
        last: -1,
        gap: 0,
    }
}

fn comb(len: i64) -> i64 {
    if len <= 2 {
        0
    } else {
        (len - 1) * (len - 2) / 2
    }
}

fn merge(a: Node, b: Node) -> Node {
    if a.cnt == 0 {
        return b;
    }
    if b.cnt == 0 {
        return a;
    }
    Node {
        cnt: a.cnt + b.cnt,
        first: a.first,
        last: b.last,
        gap: a.gap + b.gap + comb((b.first - a.last + 1) as i64),
    }
}

struct SegTree {
    n: usize,
    t: Vec<Node>,
}

impl SegTree {
    fn new(n: usize) -> Self {
        Self {
            n,
            t: vec![empty(); n * 4 + 4],
        }
    }

    fn set(&mut self, pos: usize, peak: bool) {
        self.update(0, self.n - 1, pos, peak, 1);
    }

    fn update(&mut self, l: usize, r: usize, pos: usize, peak: bool, node: usize) {
        if l == r {
            self.t[node] = if peak {
                Node {
                    cnt: 1,
                    first: pos as i32,
                    last: pos as i32,
                    gap: 0,
                }
            } else {
                empty()
            };
            return;
        }
        let mid = (l + r) / 2;
        if pos <= mid {
            self.update(l, mid, pos, peak, node * 2);
        } else {
            self.update(mid + 1, r, pos, peak, node * 2 + 1);
        }
        self.t[node] = merge(self.t[node * 2], self.t[node * 2 + 1]);
    }

    fn query(&self, ql: usize, qr: usize) -> Node {
        if ql > qr || self.n == 0 {
            return empty();
        }
        self.query_rec(1, 0, self.n - 1, ql, qr)
    }

    fn query_rec(&self, node: usize, l: usize, r: usize, ql: usize, qr: usize) -> Node {
        if ql <= l && r <= qr {
            return self.t[node];
        }
        let mid = (l + r) / 2;
        let mut ans = empty();
        if ql <= mid {
            ans = merge(ans, self.query_rec(node * 2, l, mid, ql, qr));
        }
        if qr > mid {
            ans = merge(ans, self.query_rec(node * 2 + 1, mid + 1, r, ql, qr));
        }
        ans
    }
}

fn is_peak(nums: &[i32], i: usize) -> bool {
    i > 0 && i + 1 < nums.len() && nums[i] > nums[i - 1] && nums[i] > nums[i + 1]
}

fn peak_subarrays(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
    let n = nums.len();
    let mut nums = nums;
    let mut seg = SegTree::new(n);
    for i in 1..n.saturating_sub(1) {
            if is_peak(&nums, i) {
            seg.set(i, true);
        }
    }
    let mut ans = Vec::new();
    for q in queries {
        if q[0] == 1 {
            let l = q[1] as usize;
            let r = q[2] as usize;
            if r < l + 2 {
                ans.push(0);
                continue;
            }
            let node = seg.query(l + 1, r - 1);
            if node.cnt == 0 {
                ans.push(0);
            } else {
                let total = comb((r - l + 1) as i64);
                let left = comb((node.first as i64) - (l as i64) + 1);
                let right = comb((r as i64) - (node.last as i64) + 1);
                ans.push(total - left - right - node.gap);
            }
        } else {
            let idx = q[1] as usize;
            nums[idx] = q[2];
            let from = idx.saturating_sub(1);
            let to = (idx + 1).min(n - 1);
            for i in from..=to {
                if i >= 1 && i + 1 < n {
                    seg.set(i, is_peak(&nums, i));
                }
            }
        }
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        peak_subarrays(
            vec![1, 3, 2, 4],
            vec![vec![1, 0, 3], vec![2, 1, 1], vec![1, 0, 3]]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::peak_subarrays;

    #[test]
    fn example1() {
        assert_eq!(
            peak_subarrays(
                vec![1, 3, 2, 4],
                vec![vec![1, 0, 3], vec![2, 1, 1], vec![1, 0, 3]]
            ),
            vec![2, 0]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            peak_subarrays(
                vec![9, 8, 9, 8],
                vec![vec![1, 1, 3], vec![2, 2, 1], vec![1, 0, 2]]
            ),
            vec![1, 0]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            peak_subarrays(
                vec![3, 6, 2, 7, 1],
                vec![vec![1, 1, 3], vec![2, 3, 0], vec![1, 0, 4]]
            ),
            vec![0, 3]
        );
    }
}
