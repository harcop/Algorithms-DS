/// LeetCode #3949 - Subtree Inversion Sum II
fn relax_max(slot: &mut Option<i64>, val: i64) {
    *slot = Some(match *slot {
        Some(x) => x.max(val),
        None => val,
    });
}

fn relax_min(slot: &mut Option<i64>, val: i64) {
    *slot = Some(match *slot {
        Some(x) => x.min(val),
        None => val,
    });
}

fn subtree_inversion_sum(edges: Vec<Vec<i32>>, nums: Vec<i32>, k: i32) -> i64 {
    let n = nums.len();
    let k = k as usize;
    let mut g = vec![Vec::new(); n];
    for e in edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        g[u].push(v);
        g[v].push(u);
    }
    let mut children = vec![Vec::new(); n];
    let mut parent = vec![usize::MAX; n];
    let mut stack = vec![0usize];
    let mut order = Vec::new();
    while let Some(u) = stack.pop() {
        order.push(u);
        for &v in &g[u] {
            if v == parent[u] {
                continue;
            }
            parent[v] = u;
            children[u].push(v);
            stack.push(v);
        }
    }

    let kk = k + 1;
    let mut base = vec![0i64; n];
    let mut mx: Vec<Vec<Option<i64>>> = vec![vec![None; kk]; n];
    let mut mn: Vec<Vec<Option<i64>>> = vec![vec![None; kk]; n];
    let need = k as isize - 2;

    for &u in order.iter().rev() {
        let mut empty = 0i64;
        let mut cmx = vec![None; kk];
        let mut cmn = vec![None; kk];
        for &v in &children[u] {
            let mut new_mx = vec![None; kk];
            let mut new_mn = vec![None; kk];
            let mut opts: Vec<(Option<usize>, i64, i64)> = vec![(None, base[v], base[v])];
            for t in 0..kk {
                if let Some(amx) = mx[v][t] {
                    opts.push((Some(t), amx, mn[v][t].unwrap()));
                }
            }
            for (t, amx, amn) in &opts {
                if let Some(t) = *t {
                    relax_max(&mut new_mx[t], empty + amx);
                    relax_min(&mut new_mn[t], empty + amn);
                }
            }
            for m in 0..kk {
                let Some(cur_mx) = cmx[m] else { continue };
                let cur_mn = cmn[m].unwrap();
                for (t, amx, amn) in &opts {
                    let nm =                     if let Some(tv) = *t {
                        let dm = if m >= k { k } else { m };
                        let dt = if tv >= k { k } else { tv };
                        if (dm as isize) + (dt as isize) < need {
                            continue;
                        }
                        m.min(tv)
                    } else {
                        m
                    };
                    relax_max(&mut new_mx[nm], cur_mx + amx);
                    relax_min(&mut new_mn[nm], cur_mn + amn);
                }
            }
            empty += base[v];
            cmx = new_mx;
            cmn = new_mn;
        }

        base[u] = nums[u] as i64 + empty;
        for m in 0..kk {
            let Some(val_mx) = cmx[m] else { continue };
            let val_mn = cmn[m].unwrap();
            let dist = if m + 1 >= k { k } else { m + 1 };
            relax_max(&mut mx[u][dist], nums[u] as i64 + val_mx);
            relax_min(&mut mn[u][dist], nums[u] as i64 + val_mn);
        }

        let mut inv_mx = -(nums[u] as i64);
        let mut inv_mn = -(nums[u] as i64);
        for &v in &children[u] {
            let mut cand_min = base[v];
            let mut cand_max = base[v];
            for t in (k - 1)..kk {
                if let Some(amx) = mx[v][t] {
                    cand_max = cand_max.max(amx);
                    cand_min = cand_min.min(mn[v][t].unwrap());
                }
            }
            inv_mx += -cand_min;
            inv_mn += -cand_max;
        }
        mx[u][0] = Some(inv_mx);
        mn[u][0] = Some(inv_mn);
    }

    let mut ans = base[0];
    for t in 0..kk {
        if let Some(v) = mx[0][t] {
            ans = ans.max(v);
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        subtree_inversion_sum(
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![0, 3],
                vec![1, 4],
                vec![1, 5]
            ],
            vec![1, 0, -10, 3, 4, 5],
            2
        )
    );
}

#[cfg(test)]
mod tests {
    use super::subtree_inversion_sum;

    #[test]
    fn example1() {
        assert_eq!(
            subtree_inversion_sum(
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![0, 3],
                    vec![1, 4],
                    vec![1, 5]
                ],
                vec![1, 0, -10, 3, 4, 5],
                2
            ),
            23
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            subtree_inversion_sum(vec![vec![0, 1], vec![1, 2]], vec![5, -10, -10], 1),
            25
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            subtree_inversion_sum(vec![vec![0, 1], vec![0, 2]], vec![1, -5, -6], 2),
            12
        );
    }

    #[test]
    fn example4() {
        assert_eq!(
            subtree_inversion_sum(vec![vec![0, 1], vec![0, 2]], vec![1, -5, -6], 3),
            10
        );
    }
}
