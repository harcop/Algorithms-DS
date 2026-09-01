/// LeetCode #3535 - Unit Conversion II
const MOD: i64 = 1_000_000_007;

fn mod_pow(mut a: i64, mut e: i64) -> i64 {
    let mut r = 1i64;
    a %= MOD;
    while e > 0 {
        if e & 1 == 1 {
            r = r * a % MOD;
        }
        a = a * a % MOD;
        e >>= 1;
    }
    r
}

fn query_conversions(conversions: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = conversions.len() + 1;
    let mut g = vec![Vec::new(); n];
    for c in &conversions {
        g[c[0] as usize].push((c[1] as usize, c[2] as i64));
    }
    let mut res = vec![0i64; n];
    fn dfs(s: usize, mul: i64, g: &[Vec<(usize, i64)>], res: &mut [i64]) {
        res[s] = mul;
        for &(t, w) in &g[s] {
            dfs(t, mul * w % MOD, g, res);
        }
    }
    dfs(0, 1, &g, &mut res);
    queries
        .into_iter()
        .map(|q| {
            let x = q[0] as usize;
            let y = q[1] as usize;
            (res[y] * mod_pow(res[x], MOD - 2) % MOD) as i32
        })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        query_conversions(vec![vec![0, 1, 2], vec![0, 2, 6]], vec![vec![1, 2], vec![1, 0]])
    );
}

#[cfg(test)]
mod tests {
    use super::query_conversions;

    #[test]
    fn example1() {
        assert_eq!(
            query_conversions(vec![vec![0, 1, 2], vec![0, 2, 6]], vec![vec![1, 2], vec![1, 0]]),
            vec![3, 500000004]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            query_conversions(
                vec![
                    vec![0, 1, 2],
                    vec![0, 2, 6],
                    vec![0, 3, 8],
                    vec![2, 4, 2],
                    vec![2, 5, 4],
                    vec![3, 6, 3],
                ],
                vec![vec![1, 2], vec![0, 4], vec![6, 5], vec![4, 6], vec![6, 1]]
            ),
            vec![3, 12, 1, 2, 83333334]
        );
    }
}
