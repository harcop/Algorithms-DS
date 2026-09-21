/// LeetCode #3855 - Sum of K-Digit Numbers in a Range
fn qpow(mut a: i64, mut n: i64, m: i64) -> i64 {
    let mut ans = 1i64;
    a %= m;
    while n > 0 {
        if n & 1 == 1 {
            ans = ans * a % m;
        }
        a = a * a % m;
        n >>= 1;
    }
    ans
}

fn sum_of_numbers(l: i32, r: i32, k: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = (r - l + 1) as i64;
    let sum = (l as i64 + r as i64) * n / 2 % MOD;
    let part1 = qpow(n % MOD, (k - 1) as i64, MOD);
    let part2 = (qpow(10, k as i64, MOD) - 1 + MOD) % MOD;
    let inv9 = qpow(9, MOD - 2, MOD);
    let mut ans = sum;
    ans = ans * part1 % MOD;
    ans = ans * part2 % MOD;
    ans = ans * inv9 % MOD;
    ans as i32
}

fn main() {
    println!("{}", sum_of_numbers(1, 2, 2));
}

#[cfg(test)]
mod tests {
    use super::sum_of_numbers;

    #[test]
    fn example1() {
        assert_eq!(sum_of_numbers(1, 2, 2), 66);
    }

    #[test]
    fn example2() {
        assert_eq!(sum_of_numbers(0, 1, 3), 444);
    }

    #[test]
    fn example3() {
        assert_eq!(sum_of_numbers(5, 5, 10), 555555520);
    }
}
