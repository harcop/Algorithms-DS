/// LeetCode #1716 - Calculate Money in Leetcode Bank
fn total_money(n: i32) -> i32 {
    let mut t = 0i32;
    for i in 0..n {
        t += 1 + i / 7 + i % 7;
    }
    t
}
fn main() { println!("{}", total_money(4)); }
#[cfg(test)]
mod tests {
    use super::total_money;
    #[test]
    fn example_one() {
        assert_eq!(total_money(4), 10);
    }
    #[test]
    fn example_two() {
        assert_eq!(total_money(10), 37);
    }
    #[test]
    fn example_three() {
        assert_eq!(total_money(20), 96);
    }
}
