/// LeetCode #1486 - Xor Operation In An Array
fn xor_operation(n: i32, start: i32) -> i32 {
    let mut ans = 0;
    for i in 0..n { ans ^= start + 2 * i; }
    ans
}
fn main() { println!("{}", xor_operation(5, 0)); }
#[cfg(test)]
mod tests {
    use super::xor_operation;
    #[test]
    fn example_one() { assert_eq!(xor_operation(5, 0), 8); }
    #[test]
    fn example_two() { assert_eq!(xor_operation(4, 3), 8); }
}