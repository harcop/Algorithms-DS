use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;

fn mod_pow(mut base: i64, mut exponent: usize) -> i64 {
    let mut result = 1i64;
    while exponent > 0 {
        if exponent & 1 == 1 {
            result = result * base % MOD;
        }
        base = base * base % MOD;
        exponent >>= 1;
    }
    result
}

fn combination(n: usize, k: usize) -> i64 {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    let mut combinations = vec![0i64; k + 1];
    combinations[0] = 1;
    for i in 1..=n {
        for j in (1..=k.min(i)).rev() {
            combinations[j] = (combinations[j] + combinations[j - 1]) % MOD;
        }
    }
    combinations[k]
}

/// LeetCode #2842 - Count K-Subsequences of a String With Maximum Beauty
fn count_k_subsequences_with_max_beauty(s: String, k: i32) -> i32 {
    let mut frequencies = HashMap::<u8, i64>::new();
    for byte in s.bytes() {
        *frequencies.entry(byte).or_default() += 1;
    }

    let mut k = k as usize;
    if frequencies.len() < k {
        return 0;
    }
    let mut values: Vec<_> = frequencies.into_values().collect();
    values.sort_unstable_by(|a, b| b.cmp(a));

    let threshold = values[k - 1];
    let tied = values.iter().filter(|&&value| value == threshold).count();
    let mut answer = 1i64;
    for &value in &values {
        if value == threshold {
            break;
        }
        answer = answer * value % MOD;
        k -= 1;
    }
    answer = answer * combination(tied, k) % MOD;
    answer = answer * mod_pow(threshold, k) % MOD;
    answer as i32
}

fn main() {
    println!(
        "{}",
        count_k_subsequences_with_max_beauty("bcca".into(), 2)
    );
}

#[cfg(test)]
mod tests {
    use super::count_k_subsequences_with_max_beauty;

    #[test]
    fn examples() {
        assert_eq!(
            count_k_subsequences_with_max_beauty("bcca".into(), 2),
            4
        );
        assert_eq!(
            count_k_subsequences_with_max_beauty("abbcd".into(), 4),
            2
        );
    }
}
