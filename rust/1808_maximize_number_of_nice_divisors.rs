/// LeetCode #1808 - Maximize Number of Nice Divisors
const MOD: i64 = 1_000_000_007;

fn mod_pow(mut base: i64, mut exp: i64) -> i64 {
    let mut result = 1i64;
    base %= MOD;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % MOD;
        }
        base = base * base % MOD;
        exp >>= 1;
    }
    result
}

fn max_nice_divisors(prime_factors: i32) -> i32 {
    let n = prime_factors as i64;
    if n < 4 {
        return n as i32;
    }
    match n % 3 {
        0 => mod_pow(3, n / 3) as i32,
        1 => (4 * mod_pow(3, n / 3 - 1) % MOD) as i32,
        _ => (2 * mod_pow(3, n / 3) % MOD) as i32,
    }
}

fn main() {
    println!("{}", max_nice_divisors(5));
}

#[cfg(test)]
mod tests {
    use super::max_nice_divisors;

    #[test]
    fn example_one() {
        assert_eq!(max_nice_divisors(5), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_nice_divisors(8), 18);
    }
}
