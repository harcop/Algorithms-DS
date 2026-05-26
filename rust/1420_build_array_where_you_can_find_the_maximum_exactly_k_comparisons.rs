/// LeetCode #1420 - Build Array Where You Can Find The Maximum Exactly K Comparisons
fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = n as usize;
    let m = m as i32;
    let k = k as i32;
    let mut memo = vec![vec![vec![-1i64; (k + 1) as usize]; (m + 1) as usize]; n + 1];

    fn dfs(
        i: usize,
        mx: i32,
        cnt: i32,
        n: usize,
        m: i32,
        k: i32,
        memo: &mut [Vec<Vec<i64>>],
    ) -> i64 {
        const MOD: i64 = 1_000_000_007;
        if i == n {
            return if cnt == k { 1 } else { 0 };
        }
        if cnt > k {
            return 0;
        }
        let cached = memo[i][mx as usize][cnt as usize];
        if cached >= 0 {
            return cached;
        }
        let mut ways = 0i64;
        for v in 1..=m {
            if v > mx {
                ways = (ways + dfs(i + 1, v, cnt + 1, n, m, k, memo)) % MOD;
            } else {
                ways = (ways + dfs(i + 1, mx, cnt, n, m, k, memo)) % MOD;
            }
        }
        memo[i][mx as usize][cnt as usize] = ways;
        ways
    }

    dfs(0, 0, 0, n, m, k, &mut memo) as i32
}

fn main() {
    println!("{}", num_of_arrays(2, 3, 1));
}

#[cfg(test)]
mod tests {
    use super::num_of_arrays;

    #[test]
    fn example_one() {
        assert_eq!(num_of_arrays(2, 3, 1), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_of_arrays(5, 2, 3), 0);
    }
}
