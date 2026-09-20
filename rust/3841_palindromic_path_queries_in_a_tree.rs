/// LeetCode #3841 - Palindromic Path Queries in a Tree
fn palindrome_path(
    n: i32,
    edges: Vec<Vec<i32>>,
    s: String,
    queries: Vec<String>,
) -> Vec<bool> {
    let n = n as usize;
    let mut g = vec![Vec::new(); n];
    for e in &edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        g[u].push(v);
        g[v].push(u);
    }
    let mut s = s.into_bytes();
    const LOG: usize = 17;
    let mut up = vec![[0usize; LOG]; n];
    let mut depth = vec![0i32; n];
    let mut tin = vec![0i32; n];
    let mut tout = vec![0i32; n];
    let mut timer = 0i32;
    fn dfs(
        u: usize,
        p: usize,
        g: &[Vec<usize>],
        up: &mut [[usize; LOG]],
        depth: &mut [i32],
        tin: &mut [i32],
        tout: &mut [i32],
        timer: &mut i32,
    ) {
        *timer += 1;
        tin[u] = *timer;
        for k in 1..LOG {
            up[u][k] = up[up[u][k - 1]][k - 1];
        }
        for &v in &g[u] {
            if v == p {
                continue;
            }
            up[v][0] = u;
            depth[v] = depth[u] + 1;
            dfs(v, u, g, up, depth, tin, tout, timer);
        }
        tout[u] = *timer;
    }
    up[0][0] = 0;
    dfs(0, 0, &g, &mut up, &mut depth, &mut tin, &mut tout, &mut timer);
    let lca = |mut u: usize, mut v: usize, up: &[[usize; LOG]], depth: &[i32]| -> usize {
        if depth[u] < depth[v] {
            std::mem::swap(&mut u, &mut v);
        }
        let diff = (depth[u] - depth[v]) as usize;
        for k in 0..LOG {
            if (diff >> k) & 1 == 1 {
                u = up[u][k];
            }
        }
        if u == v {
            return u;
        }
        for k in (0..LOG).rev() {
            if up[u][k] != up[v][k] {
                u = up[u][k];
                v = up[v][k];
            }
        }
        up[u][0]
    };
    let m = n as i32 + 2;
    let mut bit = vec![0i32; m as usize + 1];
    let add = |bit: &mut [i32], mut i: i32, val: i32| {
        let len = bit.len() as i32;
        while i < len {
            bit[i as usize] ^= val;
            i += i & -i;
        }
    };
    let qry = |bit: &[i32], mut i: i32| -> i32 {
        let mut r = 0;
        while i > 0 {
            r ^= bit[i as usize];
            i -= i & -i;
        }
        r
    };
    let range = |bit: &mut [i32], l: i32, r: i32, val: i32| {
        add(bit, l, val);
        add(bit, r + 1, val);
    };
    for u in 0..n {
        let val = 1 << (s[u] - b'a');
        range(&mut bit, tin[u], tout[u], val);
    }
    let mut ans = Vec::new();
    for q in queries {
        let parts: Vec<&str> = q.split_whitespace().collect();
        if parts[0] == "update" {
            let u: usize = parts[1].parse().unwrap();
            let c = parts[2].as_bytes()[0];
            let diff = (1 << (s[u] - b'a')) ^ (1 << (c - b'a'));
            if diff != 0 {
                s[u] = c;
                range(&mut bit, tin[u], tout[u], diff);
            }
        } else {
            let u: usize = parts[1].parse().unwrap();
            let v: usize = parts[2].parse().unwrap();
            let l = lca(u, v, &up, &depth);
            let mask = qry(&bit, tin[u]) ^ qry(&bit, tin[v]) ^ (1 << (s[l] - b'a'));
            ans.push(mask & (mask - 1) == 0);
        }
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        palindrome_path(
            3,
            vec![vec![0, 1], vec![1, 2]],
            "aac".into(),
            vec!["query 0 2".into(), "update 1 b".into(), "query 0 2".into()]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::palindrome_path;

    #[test]
    fn example1() {
        assert_eq!(
            palindrome_path(
                3,
                vec![vec![0, 1], vec![1, 2]],
                "aac".into(),
                vec!["query 0 2".into(), "update 1 b".into(), "query 0 2".into()]
            ),
            vec![true, false]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            palindrome_path(
                4,
                vec![vec![0, 1], vec![0, 2], vec![0, 3]],
                "abca".into(),
                vec![
                    "query 1 2".into(),
                    "update 0 b".into(),
                    "query 2 3".into(),
                    "update 3 a".into(),
                    "query 1 3".into()
                ]
            ),
            vec![false, false, true]
        );
    }
}
