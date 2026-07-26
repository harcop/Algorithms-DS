/// LeetCode #2681 - Power of Heroes
fn sum_of_power(mut nums: Vec<i32>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    nums.sort_unstable();
    let mut ans = 0i64;
    let mut p = 0i64;
    for &x in nums.iter().rev() {
        let x = x as i64;
        ans = (ans + (x * x % MOD) * x) % MOD;
        ans = (ans + x * p % MOD) % MOD;
        p = (p * 2 + x * x % MOD) % MOD;
    }
    ans as i32
}

fn main() {
    println!("{}", sum_of_power(vec![2, 1, 4]));
}

#[cfg(test)]
mod tests {
    use super::sum_of_power;

    #[test]
    fn example_one() {
        assert_eq!(sum_of_power(vec![2, 1, 4]), 141);
    }

    #[test]
    fn example_two() {
        assert_eq!(sum_of_power(vec![1, 1, 1]), 7);
    }
}
