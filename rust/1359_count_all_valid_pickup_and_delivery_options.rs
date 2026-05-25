/// LeetCode #1359 - Count All Valid Pickup And Delivery Options

fn count_orders(n: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut ans = 1i64;
    for i in 0..n {
        ans = ans * (i + 1) as i64 % MOD;
        ans = ans * (2 * i + 1) as i64 % MOD;
    }
    ans as i32
}

fn main() {
    println!("{}", count_orders(3));
}

#[cfg(test)]
mod tests {
    use super::count_orders;

    #[test]
    fn example_one() {
        assert_eq!(count_orders(3), 90);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_orders(1), 1);
    }
}
