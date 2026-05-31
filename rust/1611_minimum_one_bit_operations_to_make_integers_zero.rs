/// LeetCode #1611 - Minimum One Bit Operations To Make Integers Zero
fn minimum_one_bit_operations(n: i32) -> i32 {
    let mut x = n as u32;
    let mut ans = 0i32;
    while x > 0 {
        let k = 31 - x.leading_zeros();
        ans += (1 << k) - 1;
        x ^= 1 << k;
        if x > 0 { x ^= 1 << (k - 1); ans += 1; }
    }
    ans
}
fn main() { println!("{}", minimum_one_bit_operations(3)); }
#[cfg(test)]
mod tests {
    use super::minimum_one_bit_operations;
    #[test]
    fn example_one() { assert_eq!(minimum_one_bit_operations(3), 2); }
    #[test]
    fn example_two() { assert_eq!(minimum_one_bit_operations(6), 4); }
}