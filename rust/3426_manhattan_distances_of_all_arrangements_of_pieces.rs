/// LeetCode #3426 - Manhattan Distances of All Arrangements of Pieces
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

fn nck(n: i64, k: i64, m: i64) -> i64 {
    if k < 0 || k > n {
        return 0;
    }
    let k = k.min(n - k);
    let mut res = 1i64;
    for i in 1..=k {
        res = res * ((n - i + 1) % m) % m * mod_pow(i, m - 2, m) % m;
    }
    res
}

fn distance_sum(m: i32, n: i32, k: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let m = m as i64;
    let n = n as i64;
    let k = k as i64;
    let inv6 = mod_pow(6, MOD - 2, MOD);
    let row = n * n % MOD * ((m * m % MOD * m % MOD - m + MOD) % MOD) % MOD * inv6 % MOD;
    let col = m * m % MOD * ((n * n % MOD * n % MOD - n + MOD) % MOD) % MOD * inv6 % MOD;
    ((row + col) % MOD * nck(m * n - 2, k - 2, MOD) % MOD) as i32
}

fn main() {
    println!("{}", distance_sum(2, 2, 2));
}

#[cfg(test)]
mod tests {
    use super::distance_sum;

    #[test]
    fn example1() {
        assert_eq!(distance_sum(2, 2, 2), 8);
    }

    #[test]
    fn example2() {
        assert_eq!(distance_sum(1, 4, 3), 20);
    }
}
