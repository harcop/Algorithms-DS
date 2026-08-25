/// LeetCode #3405 - Count the Number of Arrays with K Matching Adjacent Elements
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

fn count_good_arrays(n: i32, m: i32, k: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = n as i64;
    let m = m as i64;
    let k = k as i64;
    let mx = n as usize;
    let mut fact = vec![1i64; mx];
    let mut inv = vec![1i64; mx];
    for i in 1..mx {
        fact[i] = fact[i - 1] * i as i64 % MOD;
    }
    inv[mx - 1] = mod_pow(fact[mx - 1], MOD - 2, MOD);
    for i in (1..mx).rev() {
        inv[i - 1] = inv[i] * i as i64 % MOD;
    }
    let comb = |a: i64, b: i64| -> i64 {
        if b < 0 || b > a {
            0
        } else {
            fact[a as usize] * inv[b as usize] % MOD * inv[(a - b) as usize] % MOD
        }
    };
    (comb(n - 1, k) * m % MOD * mod_pow(m - 1, n - k - 1, MOD) % MOD) as i32
}

fn main() {
    println!("{}", count_good_arrays(3, 2, 1));
}

#[cfg(test)]
mod tests {
    use super::count_good_arrays;

    #[test]
    fn example1() {
        assert_eq!(count_good_arrays(3, 2, 1), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(count_good_arrays(4, 2, 2), 6);
    }

    #[test]
    fn example3() {
        assert_eq!(count_good_arrays(5, 2, 0), 2);
    }
}
