/// LeetCode #3454 - Separate Squares II
struct SegTree {
    xs: Vec<i64>,
    cnt: Vec<i32>,
    len: Vec<i64>,
}

impl SegTree {
    fn new(xs: Vec<i64>) -> Self {
        let n = xs.len() - 1;
        let size = n * 4 + 8;
        Self {
            xs,
            cnt: vec![0; size],
            len: vec![0; size],
        }
    }

    fn modify(&mut self, u: usize, l: usize, r: usize, ql: usize, qr: usize, k: i32) {
        if ql > r || qr < l {
            return;
        }
        if ql <= l && r <= qr {
            self.cnt[u] += k;
            self.pushup(u, l, r);
            return;
        }
        let mid = (l + r) / 2;
        self.modify(u * 2, l, mid, ql, qr, k);
        self.modify(u * 2 + 1, mid + 1, r, ql, qr, k);
        self.pushup(u, l, r);
    }

    fn pushup(&mut self, u: usize, l: usize, r: usize) {
        if self.cnt[u] > 0 {
            self.len[u] = self.xs[r + 1] - self.xs[l];
        } else if l == r {
            self.len[u] = 0;
        } else {
            self.len[u] = self.len[u * 2] + self.len[u * 2 + 1];
        }
    }
}

fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
    let mut xs = Vec::new();
    let mut segs = Vec::new();
    for sq in &squares {
        let x1 = sq[0] as i64;
        let y1 = sq[1] as i64;
        let l = sq[2] as i64;
        let x2 = x1 + l;
        let y2 = y1 + l;
        xs.push(x1);
        xs.push(x2);
        segs.push((y1, x1, x2, 1i32));
        segs.push((y2, x1, x2, -1i32));
    }
    xs.sort_unstable();
    xs.dedup();
    segs.sort_unstable();
    let mut idx = std::collections::HashMap::new();
    for (i, &x) in xs.iter().enumerate() {
        idx.insert(x, i);
    }
    let n_int = xs.len() - 1;
    let mut tree = SegTree::new(xs);
    let mut area = 0i64;
    let mut y0 = 0i64;
    for &(y, x1, x2, k) in &segs {
        area += (y - y0) * tree.len[1];
        let l = idx[&x1];
        let r = idx[&x2] - 1;
        tree.modify(1, 0, n_int - 1, l, r, k);
        y0 = y;
    }
    let target = area as f64 / 2.0;
    let mut tree = SegTree::new(tree.xs.clone());
    let mut area = 0.0;
    let mut y0 = 0i64;
    for &(y, x1, x2, k) in &segs {
        let cover = tree.len[1] as f64;
        let t = (y - y0) as f64 * cover;
        if cover > 0.0 && area + t >= target {
            return y0 as f64 + (target - area) / cover;
        }
        area += t;
        let l = idx[&x1];
        let r = idx[&x2] - 1;
        tree.modify(1, 0, n_int - 1, l, r, k);
        y0 = y;
    }
    y0 as f64
}

fn main() {
    println!(
        "{}",
        separate_squares(vec![vec![0, 0, 1], vec![2, 2, 1]])
    );
}

#[cfg(test)]
mod tests {
    use super::separate_squares;

    fn close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-4, "{a} vs {b}");
    }

    #[test]
    fn example1() {
        close(separate_squares(vec![vec![0, 0, 1], vec![2, 2, 1]]), 1.0);
    }

    #[test]
    fn example2() {
        close(separate_squares(vec![vec![0, 0, 2], vec![1, 1, 1]]), 1.0);
    }
}
