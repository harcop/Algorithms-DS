/// LeetCode #4042 - Valid K-Unique Subarrays II
struct Seg {
    left: Vec<u32>,
    right: Vec<u32>,
    sum: Vec<i32>,
}

impl Seg {
    fn new() -> Self {
        Self {
            left: vec![0],
            right: vec![0],
            sum: vec![0],
        }
    }

    fn new_node(&mut self) -> u32 {
        let id = self.left.len() as u32;
        self.left.push(0);
        self.right.push(0);
        self.sum.push(0);
        id
    }

    fn update(&mut self, prev: u32, tl: i32, tr: i32, pos: i32, delta: i32) -> u32 {
        let cur = self.new_node();
        if prev != 0 {
            self.left[cur as usize] = self.left[prev as usize];
            self.right[cur as usize] = self.right[prev as usize];
            self.sum[cur as usize] = self.sum[prev as usize];
        }
        self.sum[cur as usize] += delta;
        if tl == tr {
            return cur;
        }
        let tm = tl + (tr - tl) / 2;
        if pos <= tm {
            let child = self.left[cur as usize];
            let nxt = self.update(child, tl, tm, pos, delta);
            self.left[cur as usize] = nxt;
        } else {
            let child = self.right[cur as usize];
            let nxt = self.update(child, tm + 1, tr, pos, delta);
            self.right[cur as usize] = nxt;
        }
        cur
    }

    fn query(&self, node: u32, tl: i32, tr: i32, ql: i32, qr: i32) -> i32 {
        if node == 0 || ql > tr || qr < tl {
            return 0;
        }
        if ql <= tl && tr <= qr {
            return self.sum[node as usize];
        }
        let tm = tl + (tr - tl) / 2;
        self.query(self.left[node as usize], tl, tm, ql, qr)
            + self.query(self.right[node as usize], tm + 1, tr, ql, qr)
    }
}

fn mix(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = (x ^ (x >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

fn valid_subarrays(nums: Vec<i32>, k: i32, l0: i32, r0: i32, q: i32) -> Vec<bool> {
    let n = nums.len();
    let maxv = nums.iter().copied().max().unwrap_or(0) as usize;
    let mut h1 = vec![0u64; maxv + 1];
    let mut h2 = vec![0u64; maxv + 1];
    for v in 1..=maxv {
        h1[v] = mix(v as u64) | 1;
        h2[v] = mix((v as u64) << 32 | 1) | 1;
    }
    let mut last = vec![-1i32; maxv + 1];
    let mut pref_a = vec![0u64; n + 1];
    let mut pref_b = vec![0u64; n + 1];
    let mut roots = vec![0u32; n + 1];
    let mut seg = Seg::new();
    let hi = n as i32 - 1;
    for i in 0..n {
        let v = nums[i] as usize;
        pref_a[i + 1] = pref_a[i] ^ h1[v];
        pref_b[i + 1] = pref_b[i] ^ h2[v];
        let mut root = roots[i];
        if last[v] >= 0 {
            root = seg.update(root, 0, hi, last[v], -1);
        }
        root = seg.update(root, 0, hi, i as i32, 1);
        last[v] = i as i32;
        roots[i + 1] = root;
    }
    let n_i = n as i32;
    let mut l = l0;
    let mut r = r0;
    let mut ans = Vec::with_capacity(q as usize);
    for step in 0..q {
        let ok = l <= r
            && pref_a[r as usize + 1] == pref_a[l as usize]
            && pref_b[r as usize + 1] == pref_b[l as usize]
            && seg.query(roots[r as usize + 1], 0, hi, l, r) == k;
        ans.push(ok);
        if step + 1 == q {
            break;
        }
        let g = if ok { l.wrapping_add(r) } else { r.wrapping_sub(l) };
        let mut nl = (l ^ g).rem_euclid(n_i);
        let mut nr = (r ^ g).rem_euclid(n_i);
        if nl > nr {
            std::mem::swap(&mut nl, &mut nr);
        }
        l = nl;
        r = nr;
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        valid_subarrays(vec![1, 2, 2, 1], 2, 1, 2, 2)
    );
}

#[cfg(test)]
mod tests {
    use super::valid_subarrays;

    #[test]
    fn example1() {
        assert_eq!(valid_subarrays(vec![1, 2, 2, 1], 2, 1, 2, 2), vec![false, true]);
    }

    #[test]
    fn example2() {
        assert_eq!(
            valid_subarrays(vec![1, 2, 3, 3, 4], 1, 2, 3, 2),
            vec![true, false]
        );
    }
}
