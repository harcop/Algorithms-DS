/// LeetCode #1672 - Richest Customer Wealth
fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    accounts.iter().map(|a| a.iter().sum()).max().unwrap_or(0)
}
fn main() { println!("{}", maximum_wealth(vec![vec![1,2,3],vec![3,2,1]])); }
#[cfg(test)]
mod tests {
    use super::maximum_wealth;
    #[test]
    fn example_one() { assert_eq!(maximum_wealth(vec![vec![1,2,3],vec![3,2,1]]), 6); }
}