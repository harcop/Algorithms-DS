/// LeetCode #1922 - Count Good Numbers
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

fn count_good_numbers(n: i64) -> i32 {
    (mod_pow(5, (n + 1) / 2) * mod_pow(4, n / 2) % MOD) as i32
}

fn main() {
    println!("{}", count_good_numbers(1));
}

#[cfg(test)]
mod tests {
    use super::count_good_numbers;

    #[test]
    fn example_one() {
        assert_eq!(count_good_numbers(1), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_good_numbers(4), 400);
    }
}
