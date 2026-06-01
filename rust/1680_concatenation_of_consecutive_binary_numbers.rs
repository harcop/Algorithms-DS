/// LeetCode #1680 - Concatenation Of Consecutive Binary Numbers
fn concatenated_binary(n: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut ans = 0i64;
    for i in 1..=n as i64 {
        let bits = 64 - i.leading_zeros() as i64;
        ans = ((ans << bits) + i) % MOD;
    }
    ans as i32
}
fn main() { println!("{}", concatenated_binary(3)); }
#[cfg(test)]
mod tests {
    use super::concatenated_binary;
    #[test]
    fn example_one() { assert_eq!(concatenated_binary(3), 27); }
    #[test]
    fn example_two() { assert_eq!(concatenated_binary(12), 505379714); }
}