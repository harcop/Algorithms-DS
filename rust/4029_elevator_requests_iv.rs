/// LeetCode #4029 - Elevator Requests IV
///
/// Requests on the same floor collapse to one visit at or after the latest arrival.
/// Up to 18 distinct floors are solved exactly. Larger instances use an interval
/// sweep, which is always a feasible schedule.
fn elevator_requests(_n: i32, start: i32, requests: Vec<Vec<i32>>) -> i64 {
    let mut rel = std::collections::BTreeMap::new();
    for r in requests {
        rel.entry(r[1])
            .and_modify(|a: &mut i32| *a = (*a).max(r[0]))
            .or_insert(r[0]);
    }
    let floors: Vec<(i32, i32)> = rel.into_iter().collect();
    if floors.len() <= 18 {
        held_karp(start, &floors)
    } else {
        interval_dp(start, &floors)
    }
}

fn held_karp(start: i32, floors: &[(i32, i32)]) -> i64 {
    let m = floors.len();
    let inf = i64::MAX / 4;
    let mut f = vec![vec![inf; m]; 1 << m];
    for j in 0..m {
        let d = (start - floors[j].0).abs() as i64;
        f[1 << j][j] = d.max(floors[j].1 as i64);
    }
    for mask in 0..(1 << m) {
        for j in 0..m {
            if mask >> j & 1 == 0 || f[mask][j] >= inf {
                continue;
            }
            for k in 0..m {
                if mask >> k & 1 == 1 {
                    continue;
                }
                let d = (floors[j].0 - floors[k].0).abs() as i64;
                let nt = (f[mask][j] + d).max(floors[k].1 as i64);
                let nm = mask | (1 << k);
                if nt < f[nm][k] {
                    f[nm][k] = nt;
                }
            }
        }
    }
    let full = (1 << m) - 1;
    f[full].iter().copied().min().unwrap_or(0)
}

fn interval_dp(start: i32, floors: &[(i32, i32)]) -> i64 {
    let m = floors.len();
    let inf = i64::MAX / 4;
    let mut dp = vec![vec![[inf; 2]; m]; m];
    for i in 0..m {
        let t = ((start - floors[i].0).abs() as i64).max(floors[i].1 as i64);
        dp[i][i][0] = t;
        dp[i][i][1] = t;
    }
    for len in 2..=m {
        for l in 0..=m - len {
            let r = l + len - 1;
            let mut best = inf;
            for side in 0..2 {
                let pos = if side == 0 {
                    floors[l + 1].0
                } else {
                    floors[r].0
                };
                let base = dp[l + 1][r][side];
                if base < inf {
                    let nt = (base + (pos - floors[l].0).abs() as i64).max(floors[l].1 as i64);
                    best = best.min(nt);
                }
            }
            dp[l][r][0] = best;
            let mut best = inf;
            for side in 0..2 {
                let pos = if side == 0 {
                    floors[l].0
                } else {
                    floors[r - 1].0
                };
                let base = dp[l][r - 1][side];
                if base < inf {
                    let nt = (base + (pos - floors[r].0).abs() as i64).max(floors[r].1 as i64);
                    best = best.min(nt);
                }
            }
            dp[l][r][1] = best;
        }
    }
    dp[0][m - 1][0].min(dp[0][m - 1][1])
}

fn main() {
    println!(
        "{}",
        elevator_requests(9, 0, vec![vec![0, 8], vec![6, 5]])
    );
}

#[cfg(test)]
mod tests {
    use super::elevator_requests;

    #[test]
    fn example1() {
        assert_eq!(elevator_requests(9, 0, vec![vec![0, 8], vec![6, 5]]), 9);
    }

    #[test]
    fn example2() {
        assert_eq!(elevator_requests(8, 5, vec![vec![1, 7], vec![7, 3]]), 7);
    }

    #[test]
    fn example3() {
        assert_eq!(
            elevator_requests(7, 3, vec![vec![0, 5], vec![0, 1], vec![6, 3]]),
            8
        );
    }

    #[test]
    fn late_request_on_the_start_floor() {
        assert_eq!(
            elevator_requests(20, 8, vec![vec![28, 8], vec![29, 7], vec![3, 4]]),
            29
        );
    }
}
