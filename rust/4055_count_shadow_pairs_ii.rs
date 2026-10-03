/// LeetCode #4055 - Count Shadow Pairs II
const NEG: i64 = -1_000_000_000_000_000_000;

fn count_shadow_pairs(nums: Vec<i32>) -> i64 {
    let nums: Vec<i64> = nums.into_iter().map(|x| x as i64).collect();
    solve(&nums, 0, nums.len())
}

fn solve(nums: &[i64], l: usize, r: usize) -> i64 {
    if r - l <= 1 {
        return 0;
    }
    let mid = (l + r) / 2;
    solve(nums, l, mid) + solve(nums, mid, r) + cross(nums, l, mid, r)
}

fn cross(nums: &[i64], l: usize, mid: usize, r: usize) -> i64 {
    let mut suf = vec![NEG; mid - l];
    let mut cur = NEG;
    for i in (l..mid).rev() {
        suf[i - l] = cur;
        cur = cur.max(nums[i]);
    }
    let mut pts = Vec::with_capacity(mid - l);
    let mut diag = Vec::new();
    for i in l..mid {
        let s = suf[i - l];
        let a = nums[i];
        pts.push((s, a));
        if s <= a {
            diag.push((s, a));
        }
    }
    let all = Seg::new(pts);
    let diag = Seg::new(diag);
    let mut ans = 0i64;
    let mut pref = NEG;
    for j in mid..r {
        let v = nums[j];
        if pref >= v {
            ans += all.count(None, None, None, Some(v - 1));
        } else {
            ans += all.count(None, Some(pref), Some(pref), Some(v - 1));
            ans += all.count(Some(v), None, None, Some(v - 1));
            ans += diag.count(Some(pref + 1), None, None, Some(v - 1));
        }
        pref = pref.max(v);
    }
    ans
}

struct Seg {
    pts: Vec<(i64, i64)>,
    tree: Vec<Vec<i64>>,
}

impl Seg {
    fn new(mut pts: Vec<(i64, i64)>) -> Self {
        pts.sort_unstable();
        let n = pts.len();
        let mut tree = vec![Vec::new(); n.saturating_mul(4).saturating_add(1)];
        if n > 0 {
            build(&pts, &mut tree, 1, 0, n - 1);
        }
        Self { pts, tree }
    }

    fn count(&self, s_lo: Option<i64>, s_hi: Option<i64>, a_lo: Option<i64>, a_hi: Option<i64>) -> i64 {
        let n = self.pts.len();
        if n == 0 {
            return 0;
        }
        let lo = match s_lo {
            None => 0,
            Some(s) => self.pts.partition_point(|&(x, _)| x < s),
        };
        let hi = match s_hi {
            None => n,
            Some(s) => self.pts.partition_point(|&(x, _)| x <= s),
        };
        if lo >= hi {
            return 0;
        }
        query(&self.tree, 1, 0, n - 1, lo, hi - 1, a_lo, a_hi)
    }
}

fn build(pts: &[(i64, i64)], tree: &mut [Vec<i64>], o: usize, l: usize, r: usize) {
    if l == r {
        tree[o] = vec![pts[l].1];
        return;
    }
    let m = (l + r) / 2;
    build(pts, tree, o * 2, l, m);
    build(pts, tree, o * 2 + 1, m + 1, r);
    let (left, right) = tree.split_at_mut(o * 2 + 1);
    let a = &left[o * 2];
    let b = &right[0];
    let mut merged = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    while i < a.len() || j < b.len() {
        if j == b.len() || (i < a.len() && a[i] <= b[j]) {
            merged.push(a[i]);
            i += 1;
        } else {
            merged.push(b[j]);
            j += 1;
        }
    }
    tree[o] = merged;
}

fn query(
    tree: &[Vec<i64>],
    o: usize,
    l: usize,
    r: usize,
    ql: usize,
    qr: usize,
    a_lo: Option<i64>,
    a_hi: Option<i64>,
) -> i64 {
    if qr < l || r < ql {
        return 0;
    }
    if ql <= l && r <= qr {
        let arr = &tree[o];
        let left = match a_lo {
            None => 0,
            Some(a) => arr.partition_point(|&x| x < a),
        };
        let right = match a_hi {
            None => arr.len(),
            Some(a) => arr.partition_point(|&x| x <= a),
        };
        return (right - left) as i64;
    }
    let m = (l + r) / 2;
    query(tree, o * 2, l, m, ql, qr, a_lo, a_hi)
        + query(tree, o * 2 + 1, m + 1, r, ql, qr, a_lo, a_hi)
}

fn main() {
    println!("{}", count_shadow_pairs(vec![3, 1, 4, 2, 5]));
}

#[cfg(test)]
mod tests {
    use super::count_shadow_pairs;

    #[test]
    fn example1() {
        assert_eq!(count_shadow_pairs(vec![3, 1, 4, 2, 5]), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(count_shadow_pairs(vec![6, 7, 8, 9]), 3);
    }
}
