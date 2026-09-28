/// LeetCode #4002 - Count Valid Sequences
fn count_valid_sequences(n: i32, k: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = n as usize;
    let k = k as usize;
    let mut fact = vec![1i64; n];
    for i in 1..n {
        fact[i] = fact[i - 1] * i as i64 % MOD;
    }
    let mut inv = vec![1i64; n];
    inv[n - 1] = mod_pow(fact[n - 1], MOD - 2);
    for i in (1..n).rev() {
        inv[i - 1] = inv[i] * i as i64 % MOD;
    }
    let comb = |nn: usize, kk: usize| -> i64 {
        if kk > nn {
            0
        } else {
            fact[nn] * inv[kk] % MOD * inv[nn - kk] % MOD
        }
    };
    let mut ans = comb(n - 1, k - 1);
    if (n + k) % 2 == 0 {
        ans = (ans - comb((n + k) / 2 - 1, k - 1)).rem_euclid(MOD);
    }
    ans as i32
}

fn mod_pow(mut a: i64, mut e: i64) -> i64 {
    const MOD: i64 = 1_000_000_007;
    let mut r = 1i64;
    a %= MOD;
    while e > 0 {
        if e & 1 == 1 {
            r = r * a % MOD;
        }
        a = a * a % MOD;
        e >>= 1;
    }
    r
}

fn main() {
    println!("{}", count_valid_sequences(5, 3));
}

#[cfg(test)]
mod tests {
    use super::count_valid_sequences;

    #[test]
    fn example1() {
        assert_eq!(count_valid_sequences(5, 3), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(count_valid_sequences(3, 2), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(count_valid_sequences(5, 5), 0);
    }
}
