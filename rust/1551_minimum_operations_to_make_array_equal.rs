/// LeetCode #1551 - Minimum Operations To Make Array Equal
fn min_operations(n: i32) -> i32 {
    let half = n / 2;
    if n % 2 == 0 { half * half } else { half * (half + 1) }
}
fn main() { println!("{}", min_operations(3)); }
#[cfg(test)]
mod tests {
    use super::min_operations;
    #[test]
    fn example_one() { assert_eq!(min_operations(3), 2); }
    #[test]
    fn example_two() { assert_eq!(min_operations(6), 9); }
}
