/// LeetCode #3333 - Find the Original Typed String II
fn possible_string_count(word: String, mut k: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let b = word.as_bytes();
    let n = b.len();
    let mut nums = Vec::new();
    let mut ans = 1i64;
    let mut cur = 0;
    for i in 0..n {
        cur += 1;
        if i == n - 1 || b[i] != b[i + 1] {
            if cur > 1 {
                if k > 0 {
                    nums.push(cur - 1);
                }
                ans = ans * cur as i64 % MOD;
            }
            cur = 0;
            k -= 1;
        }
    }
    if k < 1 {
        return ans as i32;
    }
    let k = k as usize;
    let m = nums.len();
    let mut f = vec![vec![0i64; k]; m + 1];
    f[0][0] = 1;
    for (i, &x) in nums.iter().enumerate() {
        let mut s = vec![0i64; k + 1];
        for j in 0..k {
            s[j + 1] = (s[j] + f[i][j]) % MOD;
        }
        for j in 0..k {
            let lo = j - x.min(j);
            f[i + 1][j] = (s[j + 1] - s[lo] + MOD) % MOD;
        }
    }
    let invalid: i64 = f[m].iter().sum::<i64>() % MOD;
    ((ans - invalid + MOD) % MOD) as i32
}

fn main() {
    println!("{}", possible_string_count("aabbccdd".into(), 7));
}

#[cfg(test)]
mod tests {
    use super::possible_string_count;

    #[test]
    fn example1() {
        assert_eq!(possible_string_count("aabbccdd".into(), 7), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(possible_string_count("aabbccdd".into(), 8), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(possible_string_count("aaabbb".into(), 3), 8);
    }
}
