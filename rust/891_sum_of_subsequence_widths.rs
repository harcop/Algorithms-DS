/// LeetCode #891 - Sum of Subsequence Widths
fn sum_subseq_widths(nums: Vec<i32>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut a: Vec<i64> = nums.into_iter().map(|x| x as i64).collect();
    a.sort_unstable();
    let n = a.len();
    let mut pow2 = vec![1i64; n];
    for i in 1..n {
        pow2[i] = (pow2[i - 1] * 2) % MOD;
    }
    let mut ans = 0i64;
    for i in 0..n {
        ans = (ans + a[i] * (pow2[i] - pow2[n - 1 - i] + MOD)) % MOD;
    }
    ans as i32
}

fn main() {
    println!("{}", sum_subseq_widths(vec![2, 1, 3]));
}

#[cfg(test)]
mod tests {
    use super::sum_subseq_widths;

    #[test]
    fn example_one() {
        assert_eq!(sum_subseq_widths(vec![2, 1, 3]), 6);
    }
}
