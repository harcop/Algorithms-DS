/// LeetCode #1575 - Count All Possible Routes
fn count_routes(fuel: Vec<i32>) -> i32 {
    let n = fuel.len();
    let mut memo = vec![vec![vec![-1i64; 201]; 1 << n]; n];
    const MOD: i64 = 1_000_000_007;

    fn dfs(
        pos: usize,
        mask: usize,
        rem: i32,
        fuel: &[i32],
        n: usize,
        memo: &mut [Vec<Vec<i64>>],
    ) -> i64 {
        if mask == (1 << n) - 1 {
            return 1;
        }
        if rem < 0 {
            return 0;
        }
        if memo[pos][mask][rem as usize] >= 0 {
            return memo[pos][mask][rem as usize];
        }
        let mut ans = 0i64;
        for nxt in 0..n {
            if mask & (1 << nxt) != 0 {
                continue;
            }
            let cost = (pos as i32 - nxt as i32).abs();
            if rem < cost {
                continue;
            }
            ans = (ans + dfs(nxt, mask | (1 << nxt), fuel[nxt] - cost, fuel, n, memo)) % MOD;
        }
        memo[pos][mask][rem as usize] = ans;
        ans
    }

    dfs(0, 1, fuel[0], &fuel, n, &mut memo) as i32
}

fn main() {
    println!("{}", count_routes(vec![2, 3, 6, 8]));
}

#[cfg(test)]
mod tests {
    use super::count_routes;

    #[test]
    fn example_one() {
        assert_eq!(count_routes(vec![2, 3, 6, 8]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_routes(vec![3, 4, 3]), 2);
    }
}
