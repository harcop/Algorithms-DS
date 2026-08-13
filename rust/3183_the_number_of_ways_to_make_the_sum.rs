/// LeetCode #3183 - The Number of Ways to Make the Sum
fn number_of_ways(n: i32) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let n = n as usize;
    let mut f = vec![0i32; n + 1];
    f[0] = 1;
    for x in [1usize, 2, 6] {
        for j in x..=n {
            f[j] = (f[j] + f[j - x]) % MOD;
        }
    }
    let mut ans = f[n];
    if n >= 4 {
        ans = (ans + f[n - 4]) % MOD;
    }
    if n >= 8 {
        ans = (ans + f[n - 8]) % MOD;
    }
    ans
}

fn main() {
    println!("{}", number_of_ways(4));
}

#[cfg(test)]
mod tests {
    use super::number_of_ways;

    #[test]
    fn example1() {
        assert_eq!(number_of_ways(4), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(number_of_ways(12), 22);
    }

    #[test]
    fn example3() {
        assert_eq!(number_of_ways(5), 4);
    }
}
