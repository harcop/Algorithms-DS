/// LeetCode #1702 - Maximum Binary String After Change
fn maximum_binary_string(binary: String) -> String {
    let n = binary.len();
    let ones = binary.bytes().filter(|&c| c == b'1').count();
    if ones == n { return binary; }
    let mut ans = vec![b'1'; n];
    ans[n - ones - 1] = b'0';
    String::from_utf8(ans).unwrap()
}
fn main() { println!("{}", maximum_binary_string("000110".into())); }
#[cfg(test)]
mod tests {
    use super::maximum_binary_string;
    #[test]
    fn example_one() { assert_eq!(maximum_binary_string("000110".into()), "111011"); }
}