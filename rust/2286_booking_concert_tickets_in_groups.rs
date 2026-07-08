/// LeetCode #2286 - Booking Concert Tickets in Groups
///
/// Segment tree tracking:
/// - `mx`: maximum remaining seats in any row in range
/// - `sum`: total remaining seats in range
#[derive(Debug)]
struct SegTree {
    n: usize,
    mx: Vec<i64>,
    sum: Vec<i64>,
}

#[allow(dead_code)]
impl SegTree {
    fn new(n: usize, init: i64) -> Self {
        let mut st = Self {
            n,
            mx: vec![0; 4 * n.max(1)],
            sum: vec![0; 4 * n.max(1)],
        };
        if n > 0 {
            st.build(1, 0, n - 1, init);
        }
        st
    }

    fn build(&mut self, u: usize, l: usize, r: usize, init: i64) {
        if l == r {
            self.mx[u] = init;
            self.sum[u] = init;
            return;
        }
        let mid = (l + r) / 2;
        self.build(u * 2, l, mid, init);
        self.build(u * 2 + 1, mid + 1, r, init);
        self.pull(u);
    }

    fn pull(&mut self, u: usize) {
        self.mx[u] = self.mx[u * 2].max(self.mx[u * 2 + 1]);
        self.sum[u] = self.sum[u * 2] + self.sum[u * 2 + 1];
    }

    fn update(&mut self, idx: usize, new_val: i64) {
        if self.n == 0 {
            return;
        }
        self.update_rec(1, 0, self.n - 1, idx, new_val);
    }

    fn update_rec(&mut self, u: usize, l: usize, r: usize, idx: usize, new_val: i64) {
        if l == r {
            self.mx[u] = new_val;
            self.sum[u] = new_val;
            return;
        }
        let mid = (l + r) / 2;
        if idx <= mid {
            self.update_rec(u * 2, l, mid, idx, new_val);
        } else {
            self.update_rec(u * 2 + 1, mid + 1, r, idx, new_val);
        }
        self.pull(u);
    }

    fn query_sum(&self, ql: usize, qr: usize) -> i64 {
        if self.n == 0 || ql > qr {
            return 0;
        }
        self.query_sum_rec(1, 0, self.n - 1, ql, qr)
    }

    fn query_sum_rec(&self, u: usize, l: usize, r: usize, ql: usize, qr: usize) -> i64 {
        if ql <= l && r <= qr {
            return self.sum[u];
        }
        let mid = (l + r) / 2;
        let mut ans = 0;
        if ql <= mid {
            ans += self.query_sum_rec(u * 2, l, mid, ql, qr.min(mid));
        }
        if qr > mid {
            ans += self.query_sum_rec(u * 2 + 1, mid + 1, r, ql.max(mid + 1), qr);
        }
        ans
    }

    fn query_row(&self, idx: usize) -> i64 {
        self.query_sum(idx, idx)
    }

    fn first_with_mx_at_least(&self, qr: usize, k: i64) -> Option<usize> {
        if self.n == 0 {
            return None;
        }
        if self.mx[1] < k {
            return None;
        }
        self.first_with_mx_at_least_rec(1, 0, self.n - 1, qr, k)
    }

    fn first_with_mx_at_least_rec(
        &self,
        u: usize,
        l: usize,
        r: usize,
        qr: usize,
        k: i64,
    ) -> Option<usize> {
        if l > qr || self.mx[u] < k {
            return None;
        }
        if l == r {
            return Some(l);
        }
        let mid = (l + r) / 2;
        if let Some(left) = self.first_with_mx_at_least_rec(u * 2, l, mid, qr, k) {
            return Some(left);
        }
        self.first_with_mx_at_least_rec(u * 2 + 1, mid + 1, r, qr, k)
    }

    fn first_with_sum_positive(&self, qr: usize) -> Option<usize> {
        if self.n == 0 {
            return None;
        }
        if self.query_sum(0, qr) == 0 {
            return None;
        }
        self.first_with_sum_positive_rec(1, 0, self.n - 1, qr)
    }

    fn first_with_sum_positive_rec(&self, u: usize, l: usize, r: usize, qr: usize) -> Option<usize> {
        if l > qr || self.sum[u] == 0 {
            return None;
        }
        if l == r {
            return Some(l);
        }
        let mid = (l + r) / 2;
        if let Some(left) = self.first_with_sum_positive_rec(u * 2, l, mid, qr) {
            return Some(left);
        }
        self.first_with_sum_positive_rec(u * 2 + 1, mid + 1, r, qr)
    }
}

#[derive(Debug)]
struct BookMyShow {
    n: usize,
    m: i64,
    seg: SegTree,
}

#[allow(dead_code)]
impl BookMyShow {
    fn new(n: i32, m: i32) -> Self {
        let n = n as usize;
        let m = m as i64;
        Self {
            n,
            m,
            seg: SegTree::new(n, m),
        }
    }

    fn gather(&mut self, k: i32, max_row: i32) -> Vec<i32> {
        let k = k as i64;
        let max_row = (max_row as usize).min(self.n.saturating_sub(1));
        let Some(r) = self.seg.first_with_mx_at_least(max_row, k) else {
            return vec![];
        };
        let rem = self.seg.query_row(r);
        let start_seat = self.m - rem;
        self.seg.update(r, rem - k);
        vec![r as i32, start_seat as i32]
    }

    fn scatter(&mut self, k: i32, max_row: i32) -> bool {
        let mut k = k as i64;
        let max_row = (max_row as usize).min(self.n.saturating_sub(1));
        if self.seg.query_sum(0, max_row) < k {
            return false;
        }

        while k > 0 {
            let Some(r) = self.seg.first_with_sum_positive(max_row) else {
                break;
            };
            let rem = self.seg.query_row(r);
            let take = rem.min(k);
            self.seg.update(r, rem - take);
            k -= take;
        }
        true
    }
}

fn main() {
    let mut bms = BookMyShow::new(2, 5);
    println!("{:?}", bms.gather(4, 0));
}

#[cfg(test)]
mod tests {
    use super::BookMyShow;

    #[test]
    fn basic_flow() {
        let mut bms = BookMyShow::new(2, 5);
        assert_eq!(bms.gather(4, 0), vec![0, 0]);
        assert_eq!(bms.gather(2, 0), Vec::<i32>::new());
        assert!(bms.scatter(5, 1));
        assert_eq!(bms.gather(5, 1), Vec::<i32>::new());
    }
}

