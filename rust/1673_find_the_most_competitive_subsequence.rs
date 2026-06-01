/// LeetCode #1673 - Find The Most Competitive Subsequence
fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = nums.len();
    let mut ans = vec![];
    for (i, &x) in nums.iter().enumerate() {
        while let Some(&last) = ans.last() {
            if last > x && ans.len() + (n - i) > k {
                ans.pop();
            } else { break; }
        }
        if ans.len() < k { ans.push(x); }
    }
    ans
}
fn main() { println!("{:?}", most_competitive(vec![3,5,2,6], 2)); }
#[cfg(test)]
mod tests {
    use super::most_competitive;
    #[test]
    fn example_one() { assert_eq!(most_competitive(vec![3,5,2,6], 2), vec![2,6]); }
}