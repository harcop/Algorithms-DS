/// LeetCode #1434 - Number Of Ways To Wear Different Hats To Each Other
fn number_ways(hats: Vec<Vec<i32>>) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let n = hats.len();
    let m = hats.iter().flatten().copied().max().unwrap_or(0) as usize;
    let mut g = vec![vec![]; m + 1];
    for (i, list) in hats.iter().enumerate() {
        for &v in list {
            g[v as usize].push(i);
        }
    }
    let mut f = vec![vec![0i32; 1 << n]; m + 1];
    f[0][0] = 1;
    for i in 1..=m {
        for j in 0..(1 << n) {
            f[i][j] = f[i - 1][j];
            for &k in &g[i] {
                if (j >> k) & 1 == 1 {
                    f[i][j] = (f[i][j] + f[i - 1][j ^ (1 << k)]) % MOD;
                }
            }
        }
    }
    f[m][(1 << n) - 1]
}

fn main() {
    println!("{}", number_ways(vec![vec![3, 5, 1], vec![3, 5]]));
}

#[cfg(test)]
mod tests {
    use super::number_ways;

    #[test]
    fn example_one() {
        assert_eq!(number_ways(vec![vec![3, 5, 1], vec![3, 5]]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            number_ways(vec![vec![1, 2, 3, 4], vec![1, 2, 3, 4], vec![1, 2, 3, 4], vec![1, 2, 3, 4]]),
            24
        );
    }
}
