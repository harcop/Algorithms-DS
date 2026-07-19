/// LeetCode #2514 - Count Anagrams
const MOD: i64 = 1_000_000_007;

fn mod_pow(mut x: i64, mut n: i64) -> i64 {
    let mut res = 1i64;
    while n > 0 {
        if n & 1 == 1 {
            res = res * x % MOD;
        }
        x = x * x % MOD;
        n >>= 1;
    }
    res
}

fn count_anagrams(s: String) -> i32 {
    let mut ans = 1i64;
    let mut mul = 1i64;
    for w in s.split_whitespace() {
        let mut cnt = [0i64; 26];
        for (i, c) in w.bytes().enumerate() {
            let i = (i + 1) as i64;
            let idx = (c - b'a') as usize;
            cnt[idx] += 1;
            ans = ans * i % MOD;
            mul = mul * cnt[idx] % MOD;
        }
    }
    (ans * mod_pow(mul, MOD - 2) % MOD) as i32
}

fn main() {
    println!("{}", count_anagrams("too hot".to_string()));
}

#[cfg(test)]
mod tests {
    use super::count_anagrams;

    #[test]
    fn example_one() {
        assert_eq!(count_anagrams("too hot".to_string()), 18);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_anagrams("aa".to_string()), 1);
    }
}
