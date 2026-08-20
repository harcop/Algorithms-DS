/// LeetCode #3327 - Check if DFS Strings Are Palindromes
fn find_answer(parent: Vec<i32>, s: String) -> Vec<bool> {
    let n = s.len();
    let s = s.as_bytes();
    let mut g = vec![vec![]; n];
    for i in 1..n {
        g[parent[i] as usize].push(i);
    }
    let mut dfs_str = Vec::with_capacity(n);
    let mut pos = vec![(0usize, 0usize); n];
    fn dfs(
        i: usize,
        g: &[Vec<usize>],
        s: &[u8],
        dfs_str: &mut Vec<u8>,
        pos: &mut [(usize, usize)],
    ) {
        let l = dfs_str.len() + 1;
        for &j in &g[i] {
            dfs(j, g, s, dfs_str, pos);
        }
        dfs_str.push(s[i]);
        let r = dfs_str.len();
        pos[i] = (l, r);
    }
    dfs(0, &g, s, &mut dfs_str, &mut pos);

    const BASE: i64 = 13331;
    const MOD: i64 = 998_244_353;
    let hashing = |t: &[u8]| {
        let mut h = vec![0i64; t.len() + 1];
        let mut p = vec![1i64; t.len() + 1];
        for i in 1..=t.len() {
            h[i] = (h[i - 1] * BASE + t[i - 1] as i64) % MOD;
            p[i] = p[i - 1] * BASE % MOD;
        }
        (h, p)
    };
    let (h1, p1) = hashing(&dfs_str);
    let rev: Vec<u8> = dfs_str.iter().rev().copied().collect();
    let (h2, p2) = hashing(&rev);
    let query = |h: &[i64], p: &[i64], l: usize, r: usize| -> i64 {
        if l > r {
            return 0;
        }
        let mut v = (h[r] - h[l - 1] * p[r + 1 - l] % MOD) % MOD;
        if v < 0 {
            v += MOD;
        }
        v
    };
    (0..n)
        .map(|i| {
            let (l, r) = pos[i];
            let k = r - l + 1;
            if k / 2 == 0 {
                return true;
            }
            let v1 = query(&h1, &p1, l, l + k / 2 - 1);
            let v2 = query(&h2, &p2, n - r + 1, n - r + 1 + k / 2 - 1);
            v1 == v2
        })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        find_answer(vec![-1, 0, 0, 1, 1, 2], "aababa".into())
    );
}

#[cfg(test)]
mod tests {
    use super::find_answer;

    #[test]
    fn example1() {
        assert_eq!(
            find_answer(vec![-1, 0, 0, 1, 1, 2], "aababa".into()),
            vec![true, true, false, true, true, true]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            find_answer(vec![-1, 0, 0, 0, 0], "aabcb".into()),
            vec![true, true, true, true, true]
        );
    }
}
