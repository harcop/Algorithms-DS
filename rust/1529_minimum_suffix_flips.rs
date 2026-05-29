/// LeetCode #1529 - Minimum Suffix Flips
fn min_flips(target: String) -> i32 {
    let mut ans = 0;
    let mut cur = '0';
    for c in target.chars() {
        if c != cur {
            ans += 1;
            cur = c;
        }
    }
    ans
}
fn main() { println!("{}", min_flips("10111".into())); }
#[cfg(test)]
mod tests {
    use super::min_flips;
    #[test]
    fn example_one() { assert_eq!(min_flips("10111".into()), 3); }
    #[test]
    fn example_two() { assert_eq!(min_flips("101".into()), 3); }
}
