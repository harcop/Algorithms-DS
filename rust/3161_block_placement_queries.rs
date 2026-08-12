/// LeetCode #3161 - Block Placement Queries
use std::collections::BTreeSet;

struct SegTree {
    n: usize,
    tree: Vec<i32>,
}

impl SegTree {
    fn new(n: usize) -> Self {
        Self {
            n,
            tree: vec![0; 4 * n],
        }
    }

    fn update(&mut self, node: usize, l: usize, r: usize, pos: usize, val: i32) {
        if l == r {
            self.tree[node] = val;
            return;
        }
        let mid = (l + r) / 2;
        if pos <= mid {
            self.update(node * 2, l, mid, pos, val);
        } else {
            self.update(node * 2 + 1, mid + 1, r, pos, val);
        }
        self.tree[node] = self.tree[node * 2].max(self.tree[node * 2 + 1]);
    }

    fn query(&self, node: usize, l: usize, r: usize, ql: usize, qr: usize) -> i32 {
        if ql > r || qr < l {
            return 0;
        }
        if ql <= l && r <= qr {
            return self.tree[node];
        }
        let mid = (l + r) / 2;
        self.query(node * 2, l, mid, ql, qr)
            .max(self.query(node * 2 + 1, mid + 1, r, ql, qr))
    }

    fn set(&mut self, pos: usize, val: i32) {
        self.update(1, 0, self.n - 1, pos, val);
    }

    fn range_max(&self, ql: usize, qr: usize) -> i32 {
        if ql > qr {
            return 0;
        }
        self.query(1, 0, self.n - 1, ql, qr)
    }
}

fn get_results(queries: Vec<Vec<i32>>) -> Vec<bool> {
    let mx = (50000.min(queries.len() * 3) + 5) as i32;
    let mut obstacles = BTreeSet::new();
    obstacles.insert(0);
    obstacles.insert(mx);
    let mut st = SegTree::new((mx + 1) as usize);
    st.set(mx as usize, mx);
    let mut ans = Vec::new();
    for q in queries {
        if q[0] == 1 {
            let x = q[1];
            let prev = *obstacles.range(..x).next_back().unwrap();
            let next = *obstacles.range(x..).next().unwrap();
            obstacles.insert(x);
            st.set(x as usize, x - prev);
            st.set(next as usize, next - x);
        } else {
            let x = q[1];
            let sz = q[2];
            let prev = *obstacles.range(..=x).next_back().unwrap();
            let max_gap = st.range_max(0, prev as usize);
            ans.push(max_gap >= sz || x - prev >= sz);
        }
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        get_results(vec![
            vec![1, 2],
            vec![2, 3, 3],
            vec![2, 3, 1],
            vec![2, 2, 2]
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::get_results;

    #[test]
    fn example1() {
        assert_eq!(
            get_results(vec![
                vec![1, 2],
                vec![2, 3, 3],
                vec![2, 3, 1],
                vec![2, 2, 2]
            ]),
            vec![false, true, true]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            get_results(vec![
                vec![1, 7],
                vec![2, 7, 6],
                vec![1, 2],
                vec![2, 7, 5],
                vec![2, 7, 6]
            ]),
            vec![true, true, false]
        );
    }
}
