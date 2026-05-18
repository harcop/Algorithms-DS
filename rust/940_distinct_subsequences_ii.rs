/// LeetCode #940 - Distinct Subsequences II
const MOD: i64 = 1_000_000_007;

fn distinct_subseq_ii(s: String) -> i32 {
    let s = s.as_bytes();
    let mut dp = 1i64;
    let mut last = [0i64; 26];
    for &c in s {
        let i = (c - b'a') as usize;
        let add = dp;
        dp = (dp * 2 - last[i] + MOD) % MOD;
        last[i] = add;
    }
    ((dp - 1 + MOD) % MOD) as i32
}

fn main() {
    println!("{}", distinct_subseq_ii("abc".into()));
}

#[cfg(test)]
mod tests {
    use super::distinct_subseq_ii;

    #[test]
    fn example_one() {
        assert_eq!(distinct_subseq_ii("abc".into()), 7);
    }

    #[test]
    fn example_two() {
        assert_eq!(distinct_subseq_ii("aba".into()), 6);
    }

    #[test]
    fn example_three() {
        assert_eq!(distinct_subseq_ii("aaa".into()), 3);
    }
}
