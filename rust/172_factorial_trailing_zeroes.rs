/// LeetCode #172 - Factorial Trailing Zeroes
fn trailing_zeroes(n: i32) -> i32 {
    let mut ans = 0i32;
    let mut p = 5i64;
    let n = n as i64;
    while p <= n {
        ans += (n / p) as i32;
        p *= 5;
    }
    ans
}

fn main() {
    println!("{}", trailing_zeroes(5));
}

#[cfg(test)]
mod tests {
    use super::trailing_zeroes;

    #[test]
    fn example_one() {
        assert_eq!(trailing_zeroes(3), 0);
    }

    #[test]
    fn example_two() {
        assert_eq!(trailing_zeroes(5), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(trailing_zeroes(0), 0);
    }
}
