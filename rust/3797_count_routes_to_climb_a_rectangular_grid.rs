/// LeetCode #3797 - Count Routes to Climb a Rectangular Grid
fn count_routes(grid: Vec<String>, d: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = grid.len();
    let m = grid[0].len();
    let g: Vec<Vec<u8>> = grid.into_iter().map(|s| s.into_bytes()).collect();
    let d = d as i64;
    let mut dc_vert = 0i64;
    let lim = d * d - 1;
    while (dc_vert + 1) * (dc_vert + 1) <= lim {
        dc_vert += 1;
    }
    let mut dp0 = vec![0i64; m];
    let mut dp1 = vec![0i64; m];
    for c in 0..m {
        if g[n - 1][c] == b'.' {
            dp0[c] = 1;
        }
    }
    let apply_horiz = |dp0: &[i64], dp1: &mut [i64], row: usize| {
        let mut pref = vec![0i64; m + 1];
        for c in 0..m {
            pref[c + 1] = pref[c] + dp0[c];
        }
        for c in 0..m {
            if g[row][c] != b'.' {
                dp1[c] = 0;
                continue;
            }
            let lo = c.saturating_sub(d as usize);
            let hi = (c + d as usize).min(m - 1);
            let s = pref[hi + 1] - pref[lo] - dp0[c];
            dp1[c] = s.rem_euclid(MOD);
        }
    };
    apply_horiz(&dp0, &mut dp1, n - 1);
    for r in (0..n - 1).rev() {
        let mut below = vec![0i64; m];
        for c in 0..m {
            below[c] = (dp0[c] + dp1[c]) % MOD;
        }
        let mut pref = vec![0i64; m + 1];
        for c in 0..m {
            pref[c + 1] = pref[c] + below[c];
        }
        let mut ndp0 = vec![0i64; m];
        for c in 0..m {
            if g[r][c] != b'.' {
                continue;
            }
            let lo = c.saturating_sub(dc_vert as usize);
            let hi = (c + dc_vert as usize).min(m - 1);
            ndp0[c] = (pref[hi + 1] - pref[lo]).rem_euclid(MOD);
        }
        let mut ndp1 = vec![0i64; m];
        apply_horiz(&ndp0, &mut ndp1, r);
        dp0 = ndp0;
        dp1 = ndp1;
    }
    let mut ans = 0i64;
    for c in 0..m {
        ans = (ans + dp0[c] + dp1[c]) % MOD;
    }
    ans as i32
}

fn main() {
    println!("{}", count_routes(vec!["..".into(), "#.".into()], 1));
}

#[cfg(test)]
mod tests {
    use super::count_routes;

    #[test]
    fn example1() {
        assert_eq!(count_routes(vec!["..".into(), "#.".into()], 1), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(count_routes(vec!["..".into(), "#.".into()], 2), 4);
    }

    #[test]
    fn example3() {
        assert_eq!(count_routes(vec!["#".into()], 750), 0);
    }

    #[test]
    fn example4() {
        assert_eq!(count_routes(vec!["..".into()], 1), 4);
    }
}
