/// LeetCode #2834 - Find the Minimum Possible Sum of a Beautiful Array
fn minimum_possible_sum(n: i32, target: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = n as i64;
    let target = target as i64;
    let m = target / 2;
    if n <= m {
        return ((1 + n) * n / 2 % MOD) as i32;
    }
    let a = (1 + m) * m / 2 % MOD;
    let b = (target + target + n - m - 1) * (n - m) / 2 % MOD;
    ((a + b) % MOD) as i32
}

fn main() {
    println!("{}", minimum_possible_sum(2, 3));
}

#[cfg(test)]
mod tests {
    use super::minimum_possible_sum;

    #[test]
    fn examples() {
        assert_eq!(minimum_possible_sum(2, 3), 4);
        assert_eq!(minimum_possible_sum(3, 3), 8);
        assert_eq!(minimum_possible_sum(1, 1), 1);
    }
}
