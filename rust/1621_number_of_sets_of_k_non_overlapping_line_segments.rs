/// LeetCode #1621 - Number Of Sets Of K Non Overlapping Line Segments
const MOD: i64 = 1_000_000_007;

fn number_of_sets(n: i32, k: i32) -> i32 {
    let n = n as usize;
    let k = k as usize;
    let mut f = vec![vec![0i64; k + 1]; n + 1];
    let mut g = vec![vec![0i64; k + 1]; n + 1];
    f[1][0] = 1;
    for i in 2..=n {
        for j in 0..=k {
            f[i][j] = (f[i - 1][j] + g[i - 1][j]) % MOD;
            g[i][j] = g[i - 1][j];
            if j > 0 {
                g[i][j] = (g[i][j] + f[i - 1][j - 1]) % MOD;
                g[i][j] = (g[i][j] + g[i - 1][j - 1]) % MOD;
            }
        }
    }
    ((f[n][k] + g[n][k]) % MOD) as i32
}
fn main() { println!("{}", number_of_sets(4, 2)); }
#[cfg(test)]
mod tests {
    use super::number_of_sets;
    #[test]
    fn example_one() { assert_eq!(number_of_sets(4, 2), 5); }
    #[test]
    fn example_two() { assert_eq!(number_of_sets(3, 1), 3); }
}
