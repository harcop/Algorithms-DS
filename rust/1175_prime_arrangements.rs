/// LeetCode #1175 - Prime Arrangements
fn num_prime_arrangements(n: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut is_prime = vec![true; (n + 1) as usize];
    if n >= 0 {
        is_prime[0] = false;
    }
    if n >= 1 {
        is_prime[1] = false;
    }
    let mut p = 2i32;
    while (p as i64) * (p as i64) <= n as i64 {
        if is_prime[p as usize] {
            let mut q = p * p;
            while q <= n {
                is_prime[q as usize] = false;
                q += p;
            }
        }
        p += 1;
    }
    let primes = is_prime.iter().filter(|&&b| b).count() as i64;
    let composites = (n as i64) - primes;
    fn fact(x: i64) -> i64 {
        let mut r = 1i64;
        for i in 2..=x {
            r = (r * i) % MOD;
        }
        r
    }
    ((fact(primes) * fact(composites)) % MOD) as i32
}

fn main() {
    println!("{}", num_prime_arrangements(5));
}

#[cfg(test)]
mod tests {
    use super::num_prime_arrangements;

    #[test]
    fn example_one() {
        assert_eq!(num_prime_arrangements(5), 12);
    }

    #[test]
    fn example_two() {
        assert_eq!(num_prime_arrangements(100), 682289015);
    }
}
