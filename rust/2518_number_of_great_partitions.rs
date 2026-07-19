/// LeetCode #2518 - Number of Great Partitions
const MOD: i64 = 1_000_000_007;

fn count_partitions(nums: Vec<i32>, k: i32) -> i32 {
    let s: i64 = nums.iter().map(|&v| v as i64).sum();
    if s < k as i64 * 2 {
        return 0;
    }
    let n = nums.len();
    let k = k as usize;
    let mut f = vec![vec![0i64; k]; n + 1];
    f[0][0] = 1;
    let mut ans = 1i64;
    for i in 1..=n {
        let v = nums[i - 1] as usize;
        ans = ans * 2 % MOD;
        for j in 0..k {
            f[i][j] = f[i - 1][j];
            if j >= v {
                f[i][j] = (f[i][j] + f[i - 1][j - v]) % MOD;
            }
        }
    }
    for j in 0..k {
        ans = (ans - f[n][j] * 2 % MOD + MOD) % MOD;
    }
    ans as i32
}

fn main() {
    println!("{}", count_partitions(vec![1, 2, 3, 4], 4));
}

#[cfg(test)]
mod tests {
    use super::count_partitions;

    #[test]
    fn example_one() {
        assert_eq!(count_partitions(vec![1, 2, 3, 4], 4), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_partitions(vec![3, 3, 3], 4), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(count_partitions(vec![6, 6], 2), 2);
    }
}
