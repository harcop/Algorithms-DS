/// LeetCode #3428 - Maximum and Minimum Sums of at Most Size K Subsequences
fn mod_pow(mut base: i64, mut exp: i64, m: i64) -> i64 {
    if exp == 0 {
        return 1;
    }
    base %= m;
    if base < 0 {
        base += m;
    }
    let mut res = 1i64;
    while exp > 0 {
        if exp & 1 == 1 {
            res = res * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    res
}

fn min_max_sums(mut nums: Vec<i32>, k: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    nums.sort_unstable();
    let n = nums.len();
    let mut fact = vec![1i64; n + 1];
    for i in 1..=n {
        fact[i] = fact[i - 1] * i as i64 % MOD;
    }
    let mut inv = vec![1i64; n + 1];
    inv[n] = mod_pow(fact[n], MOD - 2, MOD);
    for i in (1..=n).rev() {
        inv[i - 1] = inv[i] * i as i64 % MOD;
    }
    let ncr = |a: usize, b: usize| -> i64 {
        if b > a {
            0
        } else {
            fact[a] * inv[b] % MOD * inv[a - b] % MOD
        }
    };
    let mut ans = 0i64;
    let mut cnt = 1i64;
    let km1 = (k as usize).saturating_sub(1);
    for i in 0..n {
        ans = (ans + (nums[i] as i64 + nums[n - 1 - i] as i64) % MOD * cnt) % MOD;
        cnt = (cnt * 2 - ncr(i, km1) + MOD) % MOD;
    }
    ans as i32
}

fn main() {
    println!("{}", min_max_sums(vec![1, 2, 3], 2));
}

#[cfg(test)]
mod tests {
    use super::min_max_sums;

    #[test]
    fn example1() {
        assert_eq!(min_max_sums(vec![1, 2, 3], 2), 24);
    }

    #[test]
    fn example2() {
        assert_eq!(min_max_sums(vec![5, 0, 6], 1), 22);
    }

    #[test]
    fn example3() {
        assert_eq!(min_max_sums(vec![1, 1, 1], 2), 12);
    }
}
