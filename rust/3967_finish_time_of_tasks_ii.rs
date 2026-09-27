/// LeetCode #3967 - Finish Time of Tasks II
fn min_finish_time(n: i32, edges: Vec<Vec<i32>>, base_time: Vec<i32>) -> i64 {
    let n = n as usize;
    let mut g = vec![vec![]; n];
    for e in edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        g[u].push(v);
        g[v].push(u);
    }
    let base: Vec<i64> = base_time.into_iter().map(|x| x as i64).collect();
    let mut down = vec![0i64; n];
    let mut stack = vec![(0usize, usize::MAX, 0u8)];
    while let Some((u, p, state)) = stack.pop() {
        if state == 0 {
            stack.push((u, p, 1));
            for &v in &g[u] {
                if v != p {
                    stack.push((v, u, 0));
                }
            }
        } else {
            let mut earliest = i64::MAX;
            let mut latest = i64::MIN;
            let mut has = false;
            for &v in &g[u] {
                if v != p {
                    has = true;
                    earliest = earliest.min(down[v]);
                    latest = latest.max(down[v]);
                }
            }
            down[u] = if has {
                2 * latest - earliest + base[u]
            } else {
                base[u]
            };
        }
    }

    fn combine(vals: &[i64], base: i64) -> i64 {
        if vals.is_empty() {
            return base;
        }
        let mut earliest = i64::MAX;
        let mut latest = i64::MIN;
        for &v in vals {
            earliest = earliest.min(v);
            latest = latest.max(v);
        }
        2 * latest - earliest + base
    }

    let mut ans = i64::MAX;
    let mut stack = vec![(0usize, usize::MAX, None)];
    while let Some((u, p, up)) = stack.pop() {
        let mut vals = Vec::new();
        let mut nodes = Vec::new();
        if let Some(upv) = up {
            vals.push(upv);
            nodes.push(usize::MAX);
        }
        for &v in &g[u] {
            if v != p {
                vals.push(down[v]);
                nodes.push(v);
            }
        }
        ans = ans.min(combine(&vals, base[u]));
        let m = vals.len();
        if m == 0 {
            continue;
        }
        let mut pref_mn = vec![i64::MAX; m + 1];
        let mut pref_mx = vec![i64::MIN; m + 1];
        let mut suff_mn = vec![i64::MAX; m + 1];
        let mut suff_mx = vec![i64::MIN; m + 1];
        for i in 0..m {
            pref_mn[i + 1] = pref_mn[i].min(vals[i]);
            pref_mx[i + 1] = pref_mx[i].max(vals[i]);
        }
        for i in (0..m).rev() {
            suff_mn[i] = suff_mn[i + 1].min(vals[i]);
            suff_mx[i] = suff_mx[i + 1].max(vals[i]);
        }
        for i in 0..m {
            if nodes[i] == usize::MAX {
                continue;
            }
            let upv = if m == 1 {
                base[u]
            } else {
                let mn = pref_mn[i].min(suff_mn[i + 1]);
                let mx = pref_mx[i].max(suff_mx[i + 1]);
                2 * mx - mn + base[u]
            };
            stack.push((nodes[i], u, Some(upv)));
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        min_finish_time(3, vec![vec![0, 1], vec![1, 2]], vec![9, 1, 5])
    );
}

#[cfg(test)]
mod tests {
    use super::min_finish_time;

    #[test]
    fn example1() {
        assert_eq!(
            min_finish_time(3, vec![vec![0, 1], vec![1, 2]], vec![9, 1, 5]),
            14
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            min_finish_time(3, vec![vec![0, 1], vec![0, 2]], vec![4, 7, 6]),
            12
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            min_finish_time(
                4,
                vec![vec![0, 1], vec![0, 2], vec![2, 3]],
                vec![5, 8, 2, 1]
            ),
            16
        );
    }
}
