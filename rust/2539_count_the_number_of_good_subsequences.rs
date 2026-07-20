/// LeetCode #2539 - Count the Number of Good Subsequences
fn count_good_subsequences(s: String) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut count = [0i32; 26];
    for c in s.bytes() {
        count[(c - b'a') as usize] += 1;
    }
    let max_freq = *count.iter().max().unwrap() as usize;
    let (fact, inv_fact) = get_fact_and_inv_fact(max_freq, MOD);

    let mut ans = 0i64;
    for freq in 1..=max_freq {
        let mut num = 1i64;
        for &char_freq in &count {
            if char_freq as usize >= freq {
                num = num * (1 + nck(char_freq as usize, freq, &fact, &inv_fact, MOD)) % MOD;
            }
        }
        ans = (ans + num - 1) % MOD;
    }
    ans as i32
}

fn get_fact_and_inv_fact(n: usize, m: i64) -> (Vec<i64>, Vec<i64>) {
    let mut fact = vec![0i64; n + 1];
    let mut inv_fact = vec![0i64; n + 1];
    let mut inv = vec![0i64; n + 1];
    fact[0] = 1;
    inv_fact[0] = 1;
    inv[0] = 1;
    if n >= 1 {
        inv[1] = 1;
    }
    for i in 1..=n {
        if i >= 2 {
            inv[i] = m - m / i as i64 * inv[m as usize % i] % m;
        }
        fact[i] = fact[i - 1] * i as i64 % m;
        inv_fact[i] = inv_fact[i - 1] * inv[i] % m;
    }
    (fact, inv_fact)
}

fn nck(n: usize, k: usize, fact: &[i64], inv_fact: &[i64], m: i64) -> i64 {
    fact[n] * inv_fact[k] % m * inv_fact[n - k] % m
}

fn main() {
    println!("{}", count_good_subsequences("abb".to_string()));
}

#[cfg(test)]
mod tests {
    use super::count_good_subsequences;

    #[test]
    fn example_one() {
        assert_eq!(count_good_subsequences("abb".to_string()), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_good_subsequences("aa".to_string()), 3);
    }
}
