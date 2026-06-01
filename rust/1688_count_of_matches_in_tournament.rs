/// LeetCode #1688 - Count Of Matches In Tournament
fn number_of_matches(n: i32) -> i32 {
    n - 1
}
fn main() { println!("{}", number_of_matches(7)); }
#[cfg(test)]
mod tests {
    use super::number_of_matches;
    #[test]
    fn example_one() { assert_eq!(number_of_matches(7), 6); }
}