/// LeetCode #1969 - Minimum Non-Zero Product of the Array Elements
const MOD: i64 = 1_000_000_007;

fn pow_mod(mut base: i64, mut exp: i64) -> i64 {
    let mut ans = 1i64;
    base %= MOD;
    while exp > 0 {
        if exp & 1 == 1 {
            ans = ans * base % MOD;
        }
        base = base * base % MOD;
        exp >>= 1;
    }
    ans
}

fn min_non_zero_product(p: i32) -> i32 {
    let p = p as i64;
    let a = (1i64 << p) - 1;
    let b = (1i64 << p) - 2;
    let exp = (1i64 << (p - 1)) - 1;
    (a * pow_mod(b, exp) % MOD) as i32
}

fn main() {
    println!("{}", min_non_zero_product(1));
}

#[cfg(test)]
mod tests {
    use super::min_non_zero_product;

    #[test]
    fn example_one() {
        assert_eq!(min_non_zero_product(1), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_non_zero_product(2), 6);
    }

    #[test]
    fn example_three() {
        assert_eq!(min_non_zero_product(3), 1512);
    }
}
